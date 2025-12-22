use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence, Default)]
pub enum OpenAiModels {
    Gpt4o,
    Gpt4oLatest,
    Gpt4oMini,
    Gpt4Turbo,
    #[default]
    Gpt4,
    Gpt35Turbo,
}

impl Display for OpenAiModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenAiModels::Gpt4o => write!(f, "gpt-4o"),
            OpenAiModels::Gpt4oLatest => write!(f, "chatgpt-4o-latest"),
            OpenAiModels::Gpt4oMini => write!(f, "gpt-4o-mini"),
            OpenAiModels::Gpt4Turbo => write!(f, "gpt-4-turbo"),
            OpenAiModels::Gpt4 => write!(f, "gpt-4"),
            OpenAiModels::Gpt35Turbo => write!(f, "gpt-3.5-turbo"),
        }
    }
}

impl TryFrom<&str> for OpenAiModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "gpt-4o" => Self::Gpt4o,
            "chatgpt-4o-latest" => Self::Gpt4oLatest,
            "gpt-4o-mini" => Self::Gpt4oMini,
            "gpt-4-turbo" => Self::Gpt4Turbo,
            "gpt-4" => Self::Gpt4,
            "gpt-3.5-turbo" => Self::Gpt35Turbo,
            _ => Err(AiInterfaceError::DBError(format!(
                "OpenAI unknown model {}",
                s
            )))?,
        })
    }
}
