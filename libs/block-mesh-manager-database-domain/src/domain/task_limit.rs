use chrono::{NaiveDate, Utc};
use redis::aio::MultiplexedConnection;
use redis::{AsyncCommands, RedisResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLimit {
    pub day: NaiveDate,
    pub user_id: Uuid,
    pub tasks: u64,
}

impl Into<Value> for TaskLimit {
    fn into(self) -> Value {
        let mut m: serde_json::Map<String, Value> = serde_json::Map::new();
        m.insert("day".to_string(), self.day.clone().to_string().into());
        m.insert("user_id".to_string(), self.user_id.to_string().into());
        m.insert("tasks".to_string(), self.tasks.to_string().into());
        Value::Object(m)
    }
}
impl TaskLimit {
    pub fn new(user_id: &Uuid) -> Self {
        let day = Utc::now().date_naive();
        Self {
            day,
            user_id: *user_id,
            tasks: 0,
        }
    }

    pub fn get_key(user_id: &Uuid) -> String {
        let day = Utc::now().date_naive();
        format!("{}/{}", day, user_id)
    }

    pub async fn get_value_from_redis(
        con: &mut MultiplexedConnection,
        user_id: &Uuid,
        fallback: &TaskLimit,
    ) -> anyhow::Result<Self> {
        let redis_user: String = match con.get(Self::get_key(user_id)).await {
            Ok(u) => u,
            Err(_) => return Ok(fallback.clone()),
        };
        let redis_user = match serde_json::from_str::<Self>(&redis_user) {
            Ok(u) => u,
            Err(_) => return Ok(fallback.clone()),
        };
        Ok(redis_user)
    }

    pub async fn save_user(con: &mut MultiplexedConnection, user: &Self, expire: u64) {
        if let Ok(redis_user) = serde_json::to_string(&user) {
            let _: RedisResult<()> = con
                .set_ex(
                    &Self::get_key(&user.user_id),
                    redis_user.clone(),
                    expire, // 10u64 * Backend::get_expire().await as u64,
                )
                .await;
        }
    }

    pub async fn get_task_limit(
        user_id: &Uuid,
        con: &mut MultiplexedConnection,
        limit: u64,
    ) -> anyhow::Result<Self> {
        // let limit = get_envar("TASK_LIMIT").await.parse().unwrap_or(10);
        let fallback = Self::new(user_id);
        let user: Self = Self::get_value_from_redis(con, user_id, &fallback).await?;
        if user.tasks > limit {
            Err(anyhow::anyhow!("task limit exceeded"))
        } else {
            Ok(user)
        }
    }
}
