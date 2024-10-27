use crate::database::pre_populate_db;
use crate::routes::get_router;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::connection::get_pg_pool;
use database_utils::utils::migrate::migrate;
use logger_general::tracing::setup_tracing_stdout_only;
use std::net::SocketAddr;
use std::{env, process};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod database;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    let db_pool = get_pg_pool().await;
    let env = env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT is not set");
    migrate(&db_pool, env)
        .await
        .expect("Failed to migrate database");
    let _ = pre_populate_db(&db_pool).await;
    let router = get_router();
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .nest("/", router)
        .layer(cors)
        .layer(Extension(db_pool.clone()));
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    process::exit(1);
}
