use std::{
    process::ExitCode,
    sync::{LazyLock, OnceLock},
};

use axum::Router;
use common::config::TomlConfig;
use config::SdkConfig;
use database::DbContext;
use handlers::{combo_granter, mdk_shield_api, register, risky_api};
use tracing::error;

mod config;
mod database;
mod handlers;
mod util;

struct AppState {
    db: DbContext,
    #[expect(dead_code)]
    config: &'static SdkConfig,
}

type AppStateRef = &'static AppState;

#[tokio::main]
async fn main() -> ExitCode {
    static CONFIG: LazyLock<SdkConfig> =
        LazyLock::new(|| SdkConfig::load_or_create("sdk_server.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    common::print_splash();
    common::logging::init(tracing::Level::DEBUG);
    let Ok(db) = DbContext::connect(&CONFIG.database)
        .await
        .inspect_err(|err| {
            error!(
            "Failed to connect to surrealdb. Is it configured properly and running?\nReason: {err}"
        )
        })
    else {
        return ExitCode::FAILURE;
    };

    let _ = STATE.set(AppState {
        db,
        config: &CONFIG,
    });

    let router = Router::new()
        .merge(risky_api::routes())
        .merge(register::routes())
        .merge(mdk_shield_api::routes())
        .merge(combo_granter::routes())
        .with_state(STATE.get().unwrap());

    if let Err(err) = axum_server::bind(
        CONFIG
            .http_addr
            .parse()
            .expect("invalid http_addr specified"),
    )
    .serve(router.into_make_service())
    .await
    {
        error!("axum_server::serve failed: {err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
