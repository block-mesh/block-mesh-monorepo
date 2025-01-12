use block_mesh_common::date::date_range;
use chrono::{DateTime, NaiveDate, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres, Transaction};
use std::error::Error;
use std::fmt::Display;
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TwitterTaskStatus {
    Pending,
    Assigned,
    Completed,
    Failed,
}

impl Display for TwitterTaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Assigned => write!(f, "Assigned"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
        }
    }
}

impl From<String> for TwitterTaskStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Pending" => Self::Pending,
            "Assigned" => Self::Assigned,
            "Completed" => Self::Completed,
            "Failed" => Self::Failed,
            _ => Self::Pending,
        }
    }
}

impl sqlx::Type<Postgres> for TwitterTaskStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for TwitterTaskStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for TwitterTaskStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct TwitterTask {
    pub id: Uuid,
    pub twitter_username: String,
    pub assigned_user_id: Option<Uuid>,
    pub status: TwitterTaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub since: NaiveDate,
    pub until: NaiveDate,
    pub delay: DateTime<Utc>,
    pub results: Value,
}

impl TwitterTask {
    pub fn delay() -> DateTime<Utc> {
        let mut rng = rand::thread_rng();
        let random_number: u64 = rng.gen_range(1..=900);
        let now = Utc::now();
        now + Duration::from_secs(random_number)
    }

    pub async fn get_pending_tasks(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Vec<Self>> {
        let tasks: Vec<Self> = sqlx::query_as!(
            TwitterTask,
            r#"
            SELECT
            id, assigned_user_id, twitter_username, status, created_at, updated_at, since, until, delay, results
            FROM twitter_tasks
            "#
        )
        .fetch_all(&mut **transaction)
        .await?;
        Ok(tasks)
    }

    pub async fn create_twitter_task(
        transaction: &mut Transaction<'_, Postgres>,
        twitter_username: &str,
        since: &NaiveDate,
        until: &NaiveDate,
    ) -> anyhow::Result<()> {
        let range = date_range(since, until);
        let v = Value::Null;
        let status = TwitterTaskStatus::Pending.to_string();
        for (s, u) in range {
            let delay = Self::delay();
            sqlx::query!(
                r#"
                INSERT INTO twitter_tasks
                (twitter_username, status, since, until, delay, results)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
                twitter_username,
                status,
                s,
                u,
                delay,
                v,
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }
}
