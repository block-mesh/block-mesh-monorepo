use database_utils::utils::option_uuid::OptionUuid;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DailyStatStatus {
    OnGoing,
    Finalized,
}

impl Display for DailyStatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DailyStatStatus::OnGoing => write!(f, "OnGoing"),
            DailyStatStatus::Finalized => write!(f, "Finalized"),
        }
    }
}

impl From<String> for DailyStatStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "OnGoing" => DailyStatStatus::OnGoing,
            "Finalized" => DailyStatStatus::Finalized,
            _ => DailyStatStatus::OnGoing,
        }
    }
}

impl sqlx::Type<Postgres> for DailyStatStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for DailyStatStatus {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for DailyStatStatus {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DailyStat {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tasks_count: i64,
    pub uptime: f64,
    pub ref_bonus: f64,
    pub ref_bonus_applied: bool,
    pub status: DailyStatStatus,
    pub day: Date,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DailyStatTmp {
    pub id: OptionUuid,
    pub user_id: OptionUuid,
    pub tasks_count: Option<i32>,
    pub uptime: Option<f64>,
    pub ref_bonus: Option<f64>,
    pub ref_bonus_applied: Option<bool>,
    pub status: Option<String>,
    pub day: Option<Date>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<OffsetDateTime>,
}
