use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence)]
pub enum MetaModels {
    Llama31405G,
}

impl Default for MetaModels {
    fn default() -> Self {
        Self::Llama31405G
    }
}

impl Display for MetaModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaModels::Llama31405G => write!(f, "llama3.1-405b"),
        }
    }
}

impl TryFrom<&str> for MetaModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "llama3.1-405b" => Self::Llama31405G,
            _ => Err(AiInterfaceError::DBError(format!(
                "Meta unknown model {}",
                s
            )))?,
        })
    }
}
