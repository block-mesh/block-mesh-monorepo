use crate::websocket::manager::WebSocketManager;
use block_mesh_common::env::environment::Environment;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::str::FromStr;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub websocket_manager: WebSocketManager,
    pub environment: Environment,
}

impl AppState {
    pub async fn new() -> Self {
        let environment = std::env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();

        let pg_url = std::env::var("DATABASE_URL").unwrap();
        let pg_options = PgConnectOptions::from_str(&pg_url).unwrap();
        let pool = PgPool::connect_with(pg_options).await.unwrap();

        let websocket_manager = WebSocketManager::new();
        Self {
            pool,
            environment,
            websocket_manager,
        }
    }
}
