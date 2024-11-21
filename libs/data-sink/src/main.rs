use axum::{Extension, Router};
use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY_WORKER;
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::connection::{get_pg_pool, get_unlimited_pg_pool};
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use serde_json::Value;
use std::net::SocketAddr;
use std::{env, mem, process};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;

pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

fn main() {
    let sentry_layer = env::var("SENTRY_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let sentry_url = env::var("SENTRY_DATA_SYNC").unwrap_or_default();
    let sentry_sample_rate = env::var("SENTRY_SAMPLE_RATE")
        .unwrap_or("0.1".to_string())
        .parse()
        .unwrap_or(0.1);
    if sentry_layer {
        let _guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                debug: env::var("APP_ENVIRONMENT").unwrap_or_default() == "local",
                sample_rate: sentry_sample_rate,
                traces_sample_rate: sentry_sample_rate,
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        mem::forget(_guard);
    }

    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await });
    tracing::error!("block mesh manager worker stopped, exiting with exit code 1");
    process::exit(1);
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    tracing::info!("Starting worker");
    Ok(())
}
