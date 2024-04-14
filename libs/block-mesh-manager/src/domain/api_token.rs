use std::error::Error;
use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use uuid::Uuid;

use secret::Secret;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiTokenStatus {
    Inactive,
    Active,
}

impl Display for ApiTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiTokenStatus::Active => write!(f, "Active"),
            ApiTokenStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

impl From<String> for ApiTokenStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Active" => ApiTokenStatus::Active,
            "Inactive" => ApiTokenStatus::Inactive,
            _ => ApiTokenStatus::Inactive,
        }
    }
}

impl sqlx::Type<Postgres> for ApiTokenStatus {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for ApiTokenStatus {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for ApiTokenStatus {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct ApiToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: Secret<Uuid>,
    pub status: ApiTokenStatus,
    pub created_at: DateTime<Utc>,
}
