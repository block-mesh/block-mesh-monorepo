use block_mesh_common::constants::BLOCKMESH_WS_REDIS_COUNT_KEY;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::interfaces::db_messages::DBMessage;
use block_mesh_manager_database_domain::domain::user::UserAndApiToken;
use database_utils::utils::connection::channel_pool::channel_pool;
use database_utils::utils::connection::follower_pool::follower_pool;
use database_utils::utils::connection::write_pool::write_pool;
use flume::Sender;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum WsCredsCache {
    UserNotFound,
    TokenMismatch,
    Found(UserAndApiToken),
}

#[derive(Clone)]
pub struct WsAppState {
    pub pool: PgPool,
    pub follower_pool: PgPool,
    pub channel_pool: PgPool,
    pub environment: Environment,
    pub redis: MultiplexedConnection,
    pub tx: Sender<DBMessage>,
    pub emails: Arc<Mutex<HashSet<String>>>,
    pub user_ids: Arc<Mutex<HashSet<Uuid>>>,
    pub creds_cache: Arc<Mutex<HashMap<(String, Uuid), WsCredsCache>>>,
}

impl WsAppState {
    pub async fn subscribe_light(&self, email: &str, user_id: &Uuid) {
        let mut emails = self.emails.lock().await;
        emails.insert(email.to_string());
        let mut user_ids = self.user_ids.lock().await;
        user_ids.insert(*user_id);
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.incr(BLOCKMESH_WS_REDIS_COUNT_KEY, 1).await;
    }

    pub async fn unsubscribe_light(&self, email: &str, user_id: &Uuid) {
        let mut emails = self.emails.lock().await;
        emails.remove(email);
        let mut user_ids = self.user_ids.lock().await;
        user_ids.remove(user_id);
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.decr(BLOCKMESH_WS_REDIS_COUNT_KEY, 1).await;
    }
}

impl WsAppState {
    pub async fn new(tx: Sender<DBMessage>) -> Self {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let pool = write_pool(None).await;
        let follower_pool = follower_pool(Some("FOLLOWER_DATABASE_URL".to_string())).await;
        let channel_pool = channel_pool(Some("CHANNEL_DATABASE_URL".to_string())).await;
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
            creds_cache: Arc::new(Mutex::new(HashMap::new())),
            emails: Arc::new(Mutex::new(HashSet::new())),
            user_ids: Arc::new(Mutex::new(HashSet::new())),
            pool,
            follower_pool,
            channel_pool,
            environment,
            redis,
            tx,
        }
    }
}
