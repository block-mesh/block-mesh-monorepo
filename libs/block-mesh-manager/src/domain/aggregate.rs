use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Aggregate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: AggregateName,
    pub value: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AggregateName {
    Uptime,
    Bandwidth,
    Tasks,
}

impl Display for AggregateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateName::Uptime => write!(f, "Uptime"),
            AggregateName::Bandwidth => write!(f, "Bandwidth"),
            AggregateName::Tasks => write!(f, "Tasks"),
        }
    }
}

impl From<String> for AggregateName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Uptime" => AggregateName::Uptime,
            "Bandwidth" => AggregateName::Bandwidth,
            "Tasks" => AggregateName::Tasks,
            _ => AggregateName::Uptime,
        }
    }
}

impl sqlx::Type<Postgres> for AggregateName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for AggregateName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for AggregateName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
