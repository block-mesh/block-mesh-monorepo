mod data_sink;
mod database;
mod errors;
mod routes;

use crate::routes::get_router;
use axum::Router;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::connection::get_pg_pool;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::str::FromStr;
use std::{env, mem, process};
use tokio::net::TcpListener;
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
    let sentry_url = env::var("SENTRY_DATA_SINK").unwrap_or_default();
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

#[derive(Clone)]
pub struct AppState {
    pub data_sink_db_pool: PgPool,
    pub follower_db_pool: PgPool,
    pub environment: Environment,
}

impl AppState {
    pub async fn new() -> Self {
        let data_sink_db_pool = get_pg_pool(None).await;
        let follower_db_pool = get_pg_pool(Some("FOLLOWER_DATABASE_URL".to_string())).await;
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        Self {
            data_sink_db_pool,
            follower_db_pool,
            environment,
        }
    }
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    tracing::info!("Starting worker");
    let state = AppState::new().await;
    let router = get_router(state);
    let cors = CorsLayer::permissive();
    let app = Router::new().nest("/", router).layer(cors);
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);

    tokio::select! {
        o = server_task => panic!("server task exit {:?}", o)
    }
}
