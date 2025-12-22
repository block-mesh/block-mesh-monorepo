#![allow(clippy::derivable_impls)]
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum MessageMode {
    #[default]
    ResetOnEachMessage,
    ResetOnModelChange,
    KeepAlways,
}

impl Display for MessageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageMode::ResetOnEachMessage => write!(f, "ResetOnEachMessage"),
            MessageMode::ResetOnModelChange => write!(f, "ResetOnModelChange"),
            MessageMode::KeepAlways => write!(f, "KeepAlways"),
        }
    }
}

impl From<String> for MessageMode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "ResetOnEachMessage" => MessageMode::ResetOnEachMessage,
            "ResetOnModelChange" => MessageMode::ResetOnModelChange,
            "KeepAlways" => MessageMode::KeepAlways,
            _ => MessageMode::default(),
        }
    }
}

impl From<Option<String>> for MessageMode {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => MessageMode::from(s),
            None => MessageMode::default(),
        }
    }
}

impl sqlx::Type<Postgres> for MessageMode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for MessageMode {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for MessageMode {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
