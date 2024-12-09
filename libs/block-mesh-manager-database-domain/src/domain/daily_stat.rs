use crate::domain::option_uuid::OptionUuid;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
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
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for DailyStatStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
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
    pub day: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
    pub day: Option<NaiveDate>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
