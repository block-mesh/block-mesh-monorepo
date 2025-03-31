use block_mesh_common::constants::BLOCKMESH_WS_REDIS_COUNT_KEY;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::interfaces::db_messages::DBMessage;
use block_mesh_common::solana::get_block_time;
use block_mesh_manager_database_domain::domain::twitter_task::{TwitterTask, TwitterTaskStatus};
use block_mesh_manager_database_domain::domain::user::UserAndApiToken;
use database_utils::utils::connection::channel_pool::channel_pool;
use database_utils::utils::connection::follower_pool::follower_pool;
use database_utils::utils::connection::write_pool::write_pool;
use flume::Sender;
use local_ip_address::local_ip;
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{env, process};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::sleep;
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
    pub emails: Arc<RwLock<HashSet<String>>>,
    pub user_ids: Arc<RwLock<HashSet<Uuid>>>,
    pub creds_cache: Arc<RwLock<HashMap<(String, Uuid), WsCredsCache>>>,
    pub redis_key: String,
    /// task id to task mapping
    pub pending_twitter_tasks: Arc<RwLock<HashMap<Uuid, TwitterTask>>>,
    /// user_id to task mapping, if the node confirmed scraping
    pub workers: Arc<RwLock<HashMap<Uuid, Option<TwitterTask>>>>,
    pub joiner_tx: Sender<JoinHandle<()>>,
    pub block_time: Arc<RwLock<i64>>,
}

impl WsAppState {
    pub async fn cleanup_task(&self, id: &Uuid) {
        let dur = env::var("CLEANUP_TASK_SLEEP")
            .unwrap_or("20000".to_lowercase())
            .parse()
            .unwrap_or(10_000);
        let id = *id;
        let state = self.clone();
        let state_c = self.clone();
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(dur)).await;
            if let Some(Some(task)) = state.get_worker(&id).await {
                let mut task = task.clone();
                match task.status {
                    TwitterTaskStatus::Completed => {
                        state.remove_task(&task.id).await;
                    }
                    _ => {
                        task.status = TwitterTaskStatus::Pending;
                        state.add_task(&task).await;
                    }
                }
            }
            state.reset_worker(&id).await;
        });
        let _ = state_c.joiner_tx.send_async(handle).await;
    }

    pub async fn assign_task(&self) -> Option<()> {
        let worker = self.get_available_worker().await?;
        let mut task = self.get_task().await?.clone();
        task.status = TwitterTaskStatus::Assigned;
        self.add_task(&task).await;
        self.add_worker(&worker, Some(task)).await;
        self.cleanup_task(&worker).await;
        Some(())
    }

    #[tracing::instrument(name = "redis_key", skip_all)]
    pub fn redis_key(&self) -> String {
        format!("{}_{}", BLOCKMESH_WS_REDIS_COUNT_KEY, self.redis_key)
    }

    #[tracing::instrument(name = "email_key", skip_all)]
    pub fn email_key(&self, email: &str) -> String {
        format!("emails_{}", email)
    }

    pub async fn check_email_redis(&self, email: &str) -> anyhow::Result<i32> {
        let mut redis = self.redis.clone();
        let r: Option<i32> = redis.get(self.email_key(email)).await?;
        Ok(r.unwrap_or_default())
    }

    #[tracing::instrument(name = "add_task", skip_all)]
    pub async fn add_task(&self, task: &TwitterTask) {
        let mut pending_tasks = self.pending_twitter_tasks.write().await;
        pending_tasks.insert(task.id, task.clone());
    }

    #[tracing::instrument(name = "remove_task", skip_all)]
    pub async fn remove_task(&self, id: &Uuid) {
        let mut pending_tasks = self.pending_twitter_tasks.write().await;
        pending_tasks.remove(id);
    }

    #[tracing::instrument(name = "get_task", skip_all)]
    pub async fn get_task(&self) -> Option<TwitterTask> {
        let now = Utc::now();
        let pending_tasks = self.pending_twitter_tasks.read().await;
        pending_tasks
            .iter()
            .find(|i| i.1.status == TwitterTaskStatus::Pending && now > i.1.delay)
            .map(|v| v.1.clone())
    }

    #[tracing::instrument(name = "clean_tasks", skip_all)]
    pub async fn clear_tasks(&self) {
        let mut pending_tasks = self.pending_twitter_tasks.write().await;
        let ids: Vec<Uuid> = pending_tasks
            .iter()
            .filter(|i| i.1.status != TwitterTaskStatus::Pending)
            .map(|i| *i.0)
            .collect();
        for id in ids {
            pending_tasks.remove(&id);
        }
    }

    #[tracing::instrument(name = "find_task", skip_all)]
    pub async fn find_task(&self, id: &Uuid) -> Option<TwitterTask> {
        let pending_twitter_tasks = self.pending_twitter_tasks.read().await;
        pending_twitter_tasks
            .iter()
            .find(|i| *i.0 == *id)
            .map(|t| t.1.clone())
    }

    #[tracing::instrument(name = "add_worker", skip_all)]
    pub async fn add_worker(&self, user_id: &Uuid, task: Option<TwitterTask>) {
        let mut workers = self.workers.write().await;
        workers.insert(*user_id, task);
    }

    #[tracing::instrument(name = "reset_worker", skip_all)]
    pub async fn reset_worker(&self, user_id: &Uuid) {
        let mut workers = self.workers.write().await;
        workers.insert(*user_id, None);
    }

    #[tracing::instrument(name = "remove_worker", skip_all)]
    pub async fn remove_worker(&self, user_id: &Uuid) {
        let mut workers = self.workers.write().await;
        workers.remove(user_id);
    }

    #[tracing::instrument(name = "get_worker", skip_all)]
    pub async fn get_worker(&self, user_id: &Uuid) -> Option<Option<TwitterTask>> {
        let workers = self.workers.read().await;
        workers.get(user_id).cloned()
    }

    #[tracing::instrument(name = "get_available_worker", skip_all)]
    pub async fn get_available_worker(&self) -> Option<Uuid> {
        let workers = self.workers.read().await;
        workers.iter().find(|i| i.1.is_none()).map(|w| *w.0)
    }

    #[tracing::instrument(name = "subscribe_light", skip_all)]
    pub async fn subscribe_light(&self, email: &str, user_id: &Uuid) {
        self.add_email(email).await;
        self.add_user_id(user_id).await;
        self.incr_redis().await;
        self.add_email_redis(email).await;
    }

    #[tracing::instrument(name = "unsubscribe_light", skip_all)]
    pub async fn unsubscribe_light(&self, email: &str, user_id: &Uuid) {
        self.remove_email(email).await;
        self.remove_user_id(user_id).await;
        self.remove_worker(user_id).await;
        self.decr_redis().await;
        self.remove_email_redis(email).await;
    }

    #[tracing::instrument(name = "add_email_redis", skip_all)]
    pub async fn add_email_redis(&self, email: &str) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.incr(self.email_key(email), 1).await;
        let _: RedisResult<()> = redis.expire(self.email_key(email), 40).await;
    }

    #[tracing::instrument(name = "add_email_redis", skip_all)]
    pub async fn touch_email_redis(&self, email: &str) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.expire(self.email_key(email), 40).await;
    }

    #[tracing::instrument(name = "remove_email_redis", skip_all)]
    pub async fn remove_email_redis(&self, email: &str) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.decr(self.email_key(email), 1).await;
        let _: RedisResult<()> = redis.expire(self.email_key(email), 5).await;
    }

    #[tracing::instrument(name = "init_redis", skip_all)]
    pub async fn init_redis(&self) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.set(self.redis_key(), 0).await;
        let _: RedisResult<()> = redis.expire(self.redis_key(), 120).await;
    }

    #[tracing::instrument(name = "incr_redis", skip_all)]
    pub async fn incr_redis(&self) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.incr(self.redis_key(), 1).await;
        let _: RedisResult<()> = redis.expire(self.redis_key(), 120).await;
    }

    #[tracing::instrument(name = "decr_redis", skip_all)]
    pub async fn decr_redis(&self) {
        let mut redis = self.redis.clone();
        let _: RedisResult<()> = redis.decr(self.redis_key(), 1).await;
        let _: RedisResult<()> = redis.expire(self.redis_key(), 120).await;
    }

    #[tracing::instrument(name = "add_email", skip_all)]
    pub async fn add_email(&self, email: &str) {
        let mut emails = self.emails.write().await;
        emails.insert(email.to_string());
    }

    #[tracing::instrument(name = "add_user_id", skip_all)]
    pub async fn add_user_id(&self, user_id: &Uuid) {
        let mut user_ids = self.user_ids.write().await;
        user_ids.insert(*user_id);
    }

    #[tracing::instrument(name = "remove_email", skip_all)]
    pub async fn remove_email(&self, email: &str) {
        let mut emails = self.emails.write().await;
        emails.remove(email);
    }

    #[tracing::instrument(name = "remove_user_id", skip_all)]
    pub async fn remove_user_id(&self, user_id: &Uuid) {
        let mut user_ids = self.user_ids.write().await;
        user_ids.remove(user_id);
    }

    #[tracing::instrument(name = "update_block_time", skip_all)]
    pub async fn update_block_time(&self) {
        let block_time = get_block_time().await;
        *self.block_time.write().await = block_time;
    }
}

impl WsAppState {
    #[tracing::instrument(name = "new", skip_all)]
    pub async fn new(tx: Sender<DBMessage>, joiner_tx: Sender<JoinHandle<()>>) -> Self {
        let block_time = get_block_time().await;
        let pending_twitter_tasks = Arc::new(RwLock::new(HashMap::with_capacity(500)));
        let workers = Arc::new(RwLock::new(HashMap::with_capacity(500)));
        let redis_key = format!(
            "{}_{}_{}_{}",
            Uuid::new_v4(),
            hostname::get()
                .unwrap_or("unknown_host".to_string().parse().unwrap())
                .to_str()
                .unwrap_or("unknown_host"),
            process::id(),
            local_ip().unwrap_or(IpAddr::from_str("10.0.0.0").unwrap())
        );
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
            joiner_tx,
            workers,
            pending_twitter_tasks,
            redis_key,
            creds_cache: Arc::new(RwLock::new(HashMap::new())),
            emails: Arc::new(RwLock::new(HashSet::new())),
            user_ids: Arc::new(RwLock::new(HashSet::new())),
            pool,
            follower_pool,
            channel_pool,
            environment,
            redis,
            tx,
            block_time: Arc::new(RwLock::new(block_time)),
        }
    }
}
