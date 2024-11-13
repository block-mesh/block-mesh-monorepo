use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PerkName {
    Wallet,
    Twitter,
    FounderTwitter,
    Invalid,
}

impl Display for PerkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wallet => write!(f, "wallet"),
            Self::Twitter => write!(f, "twitter"),
            Self::FounderTwitter => write!(f, "founder_twitter"),
            Self::Invalid => write!(f, "invalid"),
        }
    }
}

impl From<String> for PerkName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "wallet" => Self::Wallet,
            "twitter" => Self::Twitter,
            "founder_twitter" => Self::FounderTwitter,
            _ => Self::Invalid,
        }
    }
}

impl From<Option<String>> for PerkName {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from(s),
            None => Self::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for PerkName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for PerkName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for PerkName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Perk {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub multiplier: f64,
    pub one_time_bonus: f64,
    pub name: PerkName,
    pub data: Value,
}
