use crate::database::{load_flags_cron, pre_populate_db};
use crate::routes::get_router;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use dashmap::DashMap;
use database_utils::utils::connection::{get_pg_pool, get_unlimited_pg_pool};
use database_utils::utils::migrate::migrate;
use logger_general::tracing::setup_tracing_stdout_only;
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::select;
use tower_http::cors::CorsLayer;

mod database;
mod error;
mod routes;

#[tracing::instrument(name = "run_server", skip_all)]
pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    let db_pool = get_pg_pool(None).await;
    let env = env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT is not set");
    let unlimited_pg_pool = get_unlimited_pg_pool(None).await;
    migrate(&unlimited_pg_pool, env)
        .await
        .expect("Failed to migrate database");
    let _ = pre_populate_db(&db_pool).await;
    let router = get_router();
    let cors = CorsLayer::permissive();
    let flags_cache: DashMap<String, Value> = DashMap::new();
    let flags_cache = Arc::new(flags_cache);
    let load_flags_cron_task = tokio::spawn(load_flags_cron(flags_cache.clone(), db_pool.clone()));
    let app = Router::new()
        .nest("/", router)
        .layer(cors)
        .layer(Extension(flags_cache))
        .layer(Extension(db_pool.clone()));
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);

    select! {
       o = load_flags_cron_task => panic!("load_flags_cron_task {:?}", o),
       o = server_task => panic!("server_task {:?}", o),
    }
}
