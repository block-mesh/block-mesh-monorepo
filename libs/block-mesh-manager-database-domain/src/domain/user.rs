use database_utils::utils::option_uuid::OptionUuid;
use secret::Secret;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum UserRole {
    User,
    Admin,
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "user" => UserRole::User,
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl From<Option<String>> for UserRole {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => UserRole::from(s),
            None => UserRole::User,
        }
    }
}

impl sqlx::Type<Postgres> for UserRole {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for UserRole {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for UserRole {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub role: UserRole,
    pub password: Secret<String>,
    pub wallet_address: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub invited_by: OptionUuid,
    pub verified_email: bool,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UserAndApiToken {
    pub user_id: Uuid,
    pub token: Secret<Uuid>,
    pub email: String,
    pub password: Secret<String>,
    pub nonce: Secret<String>,
    pub wallet_address: Option<String>,
    pub verified_email: bool,
}
