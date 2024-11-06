use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum AnthropicModels {
    Claude35HaikuLatest,
    Claude35SonnetLatest,
    Claude3OpusLatest,
}

impl Default for AnthropicModels {
    fn default() -> Self {
        Self::Claude35HaikuLatest
    }
}

impl Display for AnthropicModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnthropicModels::Claude35HaikuLatest => write!(f, "claude-3-5-haiku-latest"),
            AnthropicModels::Claude35SonnetLatest => write!(f, "claude-3-5-sonnet-latest"),
            AnthropicModels::Claude3OpusLatest => write!(f, "claude-3-opus-latest"),
        }
    }
}

impl TryFrom<&str> for AnthropicModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "claude-3-5-haiku-latest" => Self::Claude35HaikuLatest,
            "claude-3-5-sonnet-latest" => Self::Claude35SonnetLatest,
            "claude-3-opus-latest" => Self::Claude3OpusLatest,
            _ => Err(AiInterfaceError::DBError(format!(
                "Anthropic unknown model {}",
                s
            )))?,
        })
    }
}
