use block_mesh_common::date::date_range;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres, Transaction};
use std::env;
use std::error::Error;
use std::fmt::Display;
use time::{Date, Duration, OffsetDateTime};
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
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for TwitterTaskStatus {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
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
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub since: Date,
    pub until: Date,
    #[serde(with = "time::serde::rfc3339")]
    pub delay: OffsetDateTime,
    pub results: Value,
}

impl TwitterTask {
    pub fn delay() -> OffsetDateTime {
        let mut rng = rand::thread_rng();
        let random_number: i64 = rng.gen_range(1..=900);
        let now = OffsetDateTime::now_utc();
        now + Duration::seconds(random_number)
    }

    pub async fn get_pending_tasks(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Vec<Self>> {
        let limit = env::var("GET_PENDING_TASKS_LIMIT")
            .unwrap_or("500".to_string())
            .parse()
            .unwrap_or(500);

        let tasks: Vec<Self> = sqlx::query_as!(
            TwitterTask,
            r#"
            SELECT
            id, assigned_user_id, twitter_username, status, created_at, updated_at, since, until, delay, results
            FROM twitter_tasks
            WHERE status = $1
            LIMIT $2
            "#,
            TwitterTaskStatus::Pending.to_string(),
            limit

        )
        .fetch_all(&mut **transaction)
        .await?;
        Ok(tasks)
    }

    pub async fn create_twitter_task(
        transaction: &mut Transaction<'_, Postgres>,
        twitter_username: &str,
        since: &Date,
        until: &Date,
    ) -> anyhow::Result<()> {
        let range = date_range(since, until);
        let v = Value::Null;
        let status = TwitterTaskStatus::Pending.to_string();
        for (s, u) in range {
            let delay_time = Self::delay();
            sqlx::query!(
                r#"
                INSERT INTO twitter_tasks
                (twitter_username, status, since, until, delay, results)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
                twitter_username,
                status,
                s,
                u,
                delay_time,
                v,
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    pub async fn update_twitter_task(
        transaction: &mut Transaction<'_, Postgres>,
        id: &Uuid,
        status: &TwitterTaskStatus,
        results: &Value,
        assigned_user_id: &Uuid,
    ) -> anyhow::Result<()> {
        let now = OffsetDateTime::now_utc();
        sqlx::query!(
            r#"
                UPDATE twitter_tasks SET
                    status = $1,
                    results = $2,
                    assigned_user_id = $3,
                    updated_at = $4
                WHERE id = $5
            "#,
            status.to_string(),
            results,
            assigned_user_id,
            now,
            id
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
