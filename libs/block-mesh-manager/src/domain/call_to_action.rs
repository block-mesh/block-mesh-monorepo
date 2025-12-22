use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CallToActionName {
    InstallExtension,
    Invalid,
}

impl Display for CallToActionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallToActionName::InstallExtension => write!(f, "install_extension"),
            CallToActionName::Invalid => write!(f, "invalid"),
        }
    }
}

impl From<String> for CallToActionName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "install_extension" => CallToActionName::InstallExtension,
            _ => CallToActionName::Invalid,
        }
    }
}

impl From<Option<String>> for CallToActionName {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => CallToActionName::from(s),
            None => CallToActionName::Invalid,
        }
    }
}

impl sqlx::Type<Postgres> for CallToActionName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for CallToActionName {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for CallToActionName {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CallToAction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub status: bool,
    pub name: CallToActionName,
}
