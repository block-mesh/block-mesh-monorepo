use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum ModelName {
    Gpt4o,
    Gpt4oLatest,
    Gpt4oMini,
    Gpt4Turbo,
    Gpt4,
    Gpt35Turbo,
}

impl Default for ModelName {
    fn default() -> Self {
        Self::Gpt35Turbo
    }
}

impl Display for ModelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ModelName::KeepAlways => write!(f, "KeepAlways"),
            ModelName::Gpt4o => write!(f, "gpt-4o"),
            ModelName::Gpt4oLatest => write!(f, "chatgpt-4o-latest"),
            ModelName::Gpt4oMini => write!(f, "gpt-4o-mini"),
            ModelName::Gpt4Turbo => write!(f, "gpt-4-turbo"),
            ModelName::Gpt4 => write!(f, "gpt-4"),
            ModelName::Gpt35Turbo => write!(f, "gpt-3.5-turbo"),
        }
    }
}

impl From<String> for ModelName {
    fn from(s: String) -> Self {
        match s.as_str() {
            "gpt-4o" => Self::Gpt4o,
            "chatgpt-4o-latest" => Self::Gpt4oLatest,
            "gpt-4o-mini" => Self::Gpt4oMini,
            "gpt-4-turbo" => Self::Gpt4Turbo,
            "gpt-4" => Self::Gpt4,
            "gpt-3.5-turbo" => Self::Gpt35Turbo,
            _ => ModelName::default(),
        }
    }
}

impl From<Option<String>> for ModelName {
    fn from(s: Option<String>) -> Self {
        match s {
            Some(s) => ModelName::from(s),
            None => ModelName::default(),
        }
    }
}

impl sqlx::Type<Postgres> for ModelName {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, Postgres> for ModelName {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as sqlx::Encode<Postgres>>::encode(self.to_string(), buf)
    }
}

impl sqlx::Decode<'_, Postgres> for ModelName {
    fn decode(
        value: <Postgres as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        let value = value.to_string();
        Ok(Self::from(value))
    }
}
