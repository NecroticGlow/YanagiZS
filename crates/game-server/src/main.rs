use std::{
    process::ExitCode,
    sync::{Arc, LazyLock, OnceLock},
    time::Duration,
};

use anyhow::{bail, Result};
use common::config::{DatabaseSettings, ServiceType, TomlConfig};
use dashmap::DashMap;
use database::DbContext;
use level::{EventConfigManager, LevelEventGraphManager};
use player_info::PlayerInfo;
use player_util::UidCounter;
use qwer_rpc::{ProtocolServiceFrontend, RpcPtcPoint, RpcPtcServiceFrontend};

use evelyn_data::{ArchiveFile, NapFileCfg};
use protocol::*;
use serde::Deserialize;
use tracing::error;

mod database;
mod level;
mod player_util;
mod remote_config;
mod rpc_ptc;
mod scene_section_util;

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub service_id: u32,
    pub server_name: String,
    pub bind_client_version: String,
    pub config_autopatch_url: String,
    pub design_data_url: String,
    pub database: DatabaseSettings,
}

impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gameserver.toml");
}

struct PlayerSession {
    pub player_uid: u64,
    pub uid_counter: UidCounter,
    pub player_info: PlayerInfo,
    pub last_save_time: u64,
    pub level_event_graph_mgr: LevelEventGraphManager,
}

pub struct Globals {
    pub filecfg: NapFileCfg,
    pub event_config_mgr: EventConfigManager,
}

static GLOBALS: OnceLock<Globals> = OnceLock::new();

static PLAYER_MAP: LazyLock<DashMap<u64, PlayerSession>> = LazyLock::new(|| DashMap::new());
static DB_CONTEXT: OnceLock<DbContext> = OnceLock::new();

const SERVICE_TYPE: ServiceType = ServiceType::GameServer;

#[tokio::main]
async fn main() -> ExitCode {
    static CONFIG: LazyLock<GameServerConfig> =
        LazyLock::new(|| GameServerConfig::load_or_create("gameserver.toml"));

    common::print_splash();
    common::logging::init(tracing::Level::DEBUG);

    if let Err(err) = init_assets(&CONFIG) {
        error!("Failed to initialize assets. Reason: {err}");
        return ExitCode::FAILURE;
    }

    if let Err(err) = init_database(&CONFIG).await {
        error!(
            "Failed to connect to surrealdb. Is it configured properly and running?\nReason: {err}"
        );
        return ExitCode::FAILURE;
    }

    if let Err(err) = init_network(&CONFIG).await {
        error!("Failed to initialize network: {err}");
        return ExitCode::FAILURE;
    }

    // sleep, service stuff is running in separate task
    tokio::time::sleep(Duration::from_secs(u64::MAX)).await;
    ExitCode::SUCCESS
}

fn init_assets(config: &'static GameServerConfig) -> Result<()> {
    static DESIGN_DATA: OnceLock<ArchiveFile> = OnceLock::new();

    let remote_cfg = remote_config::download(config);
    let design_data_blk = remote_config::download_design_data_blk(&remote_cfg.version_info);
    let main_city_script =
        remote_config::download_main_city_script_config(&remote_cfg.version_info);

    let _ = DESIGN_DATA.set(
        evelyn_data::read_archive_file(std::io::Cursor::new(&design_data_blk))
            .expect("failed to read blk file"),
    );

    GLOBALS.get_or_init(|| Globals {
        filecfg: NapFileCfg::new(&DESIGN_DATA.get().unwrap()),
        event_config_mgr: EventConfigManager::new(&main_city_script).unwrap(),
    });

    Ok(())
}

async fn init_database(config: &'static GameServerConfig) -> Result<()> {
    let db_context = DbContext::connect(&config.database).await?;
    DB_CONTEXT.get_or_init(|| db_context);
    Ok(())
}

async fn init_network(config: &'static GameServerConfig) -> Result<()> {
    static SERVICE: OnceLock<RpcPtcServiceFrontend> = OnceLock::new();
    static POINT: OnceLock<Arc<RpcPtcPoint>> = OnceLock::new();

    let environment = remote_config::download_env_config(&config.config_autopatch_url);
    let Some(listen_end_point) = environment.get_server_end_point(SERVICE_TYPE, config.service_id)
    else {
        bail!(
            "the instance [{:?}-{}] is missing from environment.json",
            SERVICE_TYPE,
            config.service_id
        );
    };

    let service = RpcPtcServiceFrontend::new(ProtocolServiceFrontend::new());
    let Ok(listen_point) = service.create_point(Some(listen_end_point)).await else {
        bail!("failed to create_point at tcp://{listen_end_point}. Is another instance of this service already running?");
    };

    rpc_ptc::register_handlers(&listen_point);

    let _ = SERVICE.set(service);
    let _ = POINT.set(listen_point);

    Ok(())
}
