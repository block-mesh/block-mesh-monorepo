use crate::websocket::manager::WebSocketManager;
use block_mesh_common::env::environment::Environment;
use block_mesh_manager_database_domain::utils::connection::get_pg_pool;
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
        let pool = get_pg_pool().await;
        let websocket_manager = WebSocketManager::new();
        Self {
            pool,
            environment,
            websocket_manager,
        }
    }
}
