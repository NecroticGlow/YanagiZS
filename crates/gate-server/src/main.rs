use std::{
    process::ExitCode,
    sync::{LazyLock, OnceLock},
    thread,
    time::Duration,
};

use common::config::{DatabaseSettings, EnvironmentConfiguration, TomlConfig};
use database::DbContext;
use remote_config::RemoteConfiguration;
use serde::Deserialize;
use tracing::{error, Level};
use udp_server::UdpServer;

mod database;
mod net;
mod packet;
mod remote_config;
mod session;
mod udp_server;

#[derive(Deserialize)]
pub struct GateServerConfig {
    pub server_name: String,
    pub bind_client_version: String,
    pub config_autopatch_url: String,
    pub design_data_url: String,
    pub database: DatabaseSettings,
}

struct AppState {
    remote_config: RemoteConfiguration,
    environment: EnvironmentConfiguration,
    db_context: DbContext,
}

impl TomlConfig for GateServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gateserver.toml");
}

#[tokio::main]
async fn main() -> ExitCode {
    static CONFIG: LazyLock<GateServerConfig> =
        LazyLock::new(|| GateServerConfig::load_or_create("gateserver.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    common::print_splash();
    common::logging::init(Level::DEBUG);

    let environment = remote_config::download_env_config(&CONFIG.config_autopatch_url);
    let config = remote_config::download(&CONFIG);
    let gateway_port = config.port;

    let Ok(db_context) = DbContext::connect(&CONFIG.database)
        .await
        .inspect_err(|err| {
            error!(
            "Failed to connect to surrealdb. Is it configured properly and running?\nReason: {err}"
        )
        })
    else {
        return ExitCode::FAILURE;
    };

    let state = STATE.get_or_init(|| AppState {
        remote_config: config,
        environment,
        db_context,
    });

    let Ok(server) = UdpServer::new(&format!("0.0.0.0:{gateway_port}"), state) else {
        error!("failed to bind at udp://0.0.0.0:{gateway_port}, is another instance of this service already running?");
        return ExitCode::FAILURE;
    };

    thread::spawn(|| server.serve());

    tokio::time::sleep(Duration::from_secs(u64::MAX)).await;
    ExitCode::SUCCESS
}
