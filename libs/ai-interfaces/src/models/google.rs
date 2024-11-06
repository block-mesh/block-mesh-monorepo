use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum GoogleModels {
    Gemini15FlashLatest,
    Gemini15ProLatest,
}

impl Default for GoogleModels {
    fn default() -> Self {
        Self::Gemini15FlashLatest
    }
}

impl Display for GoogleModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleModels::Gemini15FlashLatest => write!(f, "gemini-1.5-flash-latest"),
            GoogleModels::Gemini15ProLatest => write!(f, "gemini-1.5-pro-latest"),
        }
    }
}

impl TryFrom<&str> for GoogleModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "gemini-1.5-flash-latest" => Self::Gemini15FlashLatest,
            "gemini-1.5-pro-latest" => Self::Gemini15ProLatest,
            _ => Err(AiInterfaceError::DBError(format!(
                "Google unknown model {}",
                s
            )))?,
        })
    }
}
