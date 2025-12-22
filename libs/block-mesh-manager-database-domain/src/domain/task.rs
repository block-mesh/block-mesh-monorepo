use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskMethod {
    GET,
    POST,
}

impl From<String> for TaskMethod {
    fn from(s: String) -> Self {
        match s.as_str() {
            "GET" => TaskMethod::GET,
            "POST" => TaskMethod::POST,
            _ => TaskMethod::GET,
        }
    }
}

impl Display for TaskMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskMethod::GET => write!(f, "GET"),
            TaskMethod::POST => write!(f, "POST"),
        }
    }
}

impl sqlx::Type<Postgres> for TaskMethod {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for TaskMethod {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for TaskMethod {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Assigned,
    Completed,
    Failed,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Assigned => write!(f, "Assigned"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl From<String> for TaskStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Pending" => TaskStatus::Pending,
            "Assigned" => TaskStatus::Assigned,
            "Completed" => TaskStatus::Completed,
            "Failed" => TaskStatus::Failed,
            _ => TaskStatus::Pending,
        }
    }
}

impl sqlx::Type<Postgres> for TaskStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for TaskStatus {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for TaskStatus {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub assigned_user_id: Option<Uuid>,
    pub status: TaskStatus,
    pub response_code: Option<i32>,
    pub response_raw: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub retries_count: i32,
    pub country: String,
    pub ip: String,
    pub asn: String,
    pub colo: String,
    pub response_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTask {
    pub id: Uuid,
    pub url: String,
    pub method: TaskMethod,
    pub headers: Option<Value>,
    pub body: Option<Value>,
}
