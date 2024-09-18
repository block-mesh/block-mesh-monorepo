use crate::routes::check_token::CheckTokenResponseMap;
use crate::routes::get_token::GetTokenResponseMap;
use crate::routes::router::get_router;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use logger_general::tracing::setup_tracing_stdout_only;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod database;
mod error;
mod routes;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    let db_pool = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let router = get_router();
    let check_token_map: CheckTokenResponseMap = Arc::new(Mutex::new(HashMap::new()));
    let get_token_map: GetTokenResponseMap = Arc::new(Mutex::new(HashMap::new()));
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .nest("/", router)
        .layer(Extension(db_pool.clone()))
        .layer(Extension(check_token_map))
        .layer(Extension(get_token_map))
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_millis(
            env::var("REQUEST_TIMEOUT")
                .unwrap_or("1000".to_string())
                .parse()
                .unwrap_or(1000),
        )));
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
