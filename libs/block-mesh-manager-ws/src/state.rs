use crate::websocket::manager::WebSocketManager;
use block_mesh_common::env::environment::Environment;
use database_utils::utils::connection::get_pg_pool;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub follower_pool: PgPool,
    pub websocket_manager: WebSocketManager,
    pub environment: Environment,
    pub redis: MultiplexedConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let environment = std::env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let pool = get_pg_pool(None).await;
        let follower_pool = get_pg_pool(Some("HEROKU_POSTGRESQL_COPPER_URL".to_string())).await;
        let websocket_manager = WebSocketManager::new();
        let redis_url = env::var("REDIS_URL").unwrap();
        let redis_url = if redis_url.ends_with("#insecure") {
            redis_url
        } else {
            format!("{}#insecure", redis_url)
        };
        let redis_client = redis::Client::open(redis_url).unwrap();
        let redis = redis_client
            .get_multiplexed_async_connection()
            .await
            .unwrap();
        Self {
            pool,
            follower_pool,
            environment,
            websocket_manager,
            redis,
        }
    }
}
