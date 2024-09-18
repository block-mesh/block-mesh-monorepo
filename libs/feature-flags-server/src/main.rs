use crate::database::{migrate, pre_populate_db};
use crate::routes::get_router;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use logger_general::tracing::setup_tracing_stdout_only;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod database;
mod error;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    let db_pool = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    migrate(&db_pool).await.expect("Failed to migrate database");
    let _ = pre_populate_db(&db_pool).await;
    let router = get_router();
    let app = Router::new()
        .nest("/", router)
        .layer(Extension(db_pool.clone()));
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
