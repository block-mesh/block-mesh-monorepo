use crate::websocket::manager::WebSocketManager;
use block_mesh_common::env::environment::Environment;
use database_utils::utils::connection::channel_pool::channel_pool;
use database_utils::utils::connection::follower_pool::follower_pool;
use database_utils::utils::connection::write_pool::write_pool;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub follower_pool: PgPool,
    pub channel_pool: PgPool,
    pub websocket_manager: Arc<WebSocketManager>,
    pub environment: Environment,
    pub redis: MultiplexedConnection,
}

impl AppState {
    pub async fn new() -> Self {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let pool = write_pool(None).await;
        let follower_pool = follower_pool(Some("FOLLOWER_DATABASE_URL".to_string())).await;
        let channel_pool = channel_pool(Some("CHANNEL_DATABASE_URL".to_string())).await;
        let websocket_manager = Arc::new(WebSocketManager::new());
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
            channel_pool,
            environment,
            websocket_manager,
            redis,
        }
    }
}
