use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{Decode, Postgres, Transaction};
use std::error::Error;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum EmailType {
    ConfirmEmail,
    ResetPassword,
    Unknown,
}

impl<'de> Deserialize<'de> for EmailType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let value = Self::from(value);
        Ok(value)
    }
}

impl Display for EmailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfirmEmail => write!(f, "confirm_email"),
            Self::ResetPassword => write!(f, "reset_password"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<String> for EmailType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "confirm_email" => Self::ConfirmEmail,
            "reset_password" => Self::ResetPassword,
            _ => Self::Unknown,
        }
    }
}

impl From<Option<String>> for EmailType {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => Self::from(s),
            None => Self::Unknown,
        }
    }
}

impl sqlx::Type<Postgres> for EmailType {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for EmailType {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn Error + 'static + Send + Sync>> {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for EmailType {
    fn decode(
        value: sqlx::postgres::PgValueRef<'_>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Email {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email_type: String,
    pub email_address: String,
    pub message_id: String,
}

impl Email {
    #[allow(dead_code)]
    pub async fn create_email(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: &Uuid,
        email_type: &EmailType,
        email_address: &str,
        message_id: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO emails
            (user_id, email_type, email_address, message_id)
            VALUES
            ($1, $2, $3, $4)
            "#,
            user_id,
            email_type.to_string(),
            email_address,
            message_id,
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
