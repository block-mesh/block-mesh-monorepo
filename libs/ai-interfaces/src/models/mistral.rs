use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence, Default)]
pub enum MistralModels {
    #[default]
    MistralSmallLatest,
}

impl Display for MistralModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MistralModels::MistralSmallLatest => write!(f, "mistral-small-latest"),
        }
    }
}

impl TryFrom<&str> for MistralModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "mistral-small-latest" => Self::MistralSmallLatest,
            _ => Err(AiInterfaceError::DBError(format!(
                "Mistral unknown model {}",
                s
            )))?,
        })
    }
}
