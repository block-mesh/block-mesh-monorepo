use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Method {
    GET,
    POST,
}

impl From<String> for Method {
    fn from(s: String) -> Self {
        match s.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::GET,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
        }
    }
}

impl sqlx::Type<Postgres> for Method {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for Method {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for Method {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Pending,
    Assigned,
    Completed,
    Failed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Assigned => write!(f, "Assigned"),
            Status::Completed => write!(f, "Completed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Pending" => Status::Pending,
            "Assigned" => Status::Assigned,
            "Completed" => Status::Completed,
            "Failed" => Status::Failed,
            _ => Status::Pending,
        }
    }
}

impl sqlx::Type<Postgres> for Status {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for Status {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for Status {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
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
    pub method: Method,
    pub headers: Option<Value>,
    pub body: Option<Value>,
    pub assigned_user_id: Option<Uuid>,
    pub status: Status,
    pub response_code: Option<i32>,
    pub response_raw: Option<String>,
    pub created_at: DateTime<Utc>,
}
