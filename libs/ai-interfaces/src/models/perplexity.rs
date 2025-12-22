use crate::error::AiInterfaceError;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use sqlx::{Decode, Postgres};
use std::error::Error;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Sequence, Default)]
pub enum PerplexityModels {
    #[default]
    Llama31SonarSmall128KOnline,
    Llama31SonarLarge128KOnline,
    Llama31SonarHuge128KOnline,
    Llama31SonarSmall128KChat,
    Llama31SonarLarge128KChat,
    Llama318BInstruct,
    Llama3170BInstruct,
}

impl Display for PerplexityModels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerplexityModels::Llama31SonarSmall128KOnline => {
                write!(f, "llama-3.1-sonar-small-128k-online")
            }
            PerplexityModels::Llama31SonarLarge128KOnline => {
                write!(f, "llama-3.1-sonar-large-128k-online")
            }
            PerplexityModels::Llama31SonarHuge128KOnline => {
                write!(f, "llama-3.1-sonar-huge-128k-online")
            }
            PerplexityModels::Llama31SonarSmall128KChat => {
                write!(f, "llama-3.1-sonar-small-128k-chat")
            }
            PerplexityModels::Llama31SonarLarge128KChat => {
                write!(f, "llama-3.1-sonar-large-128k-chat")
            }
            PerplexityModels::Llama318BInstruct => write!(f, "llama-3.1-8b-instruct"),
            PerplexityModels::Llama3170BInstruct => write!(f, "llama-3.1-70b-instruct"),
        }
    }
}

impl TryFrom<&str> for PerplexityModels {
    type Error = AiInterfaceError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s {
            "llama-3.1-sonar-small-128k-online" => Self::Llama31SonarSmall128KOnline,
            "llama-3.1-sonar-large-128k-online" => Self::Llama31SonarLarge128KOnline,
            "llama-3.1-sonar-huge-128k-online" => Self::Llama31SonarHuge128KOnline,
            "llama-3.1-sonar-small-128k-chat" => Self::Llama31SonarSmall128KChat,
            "llama-3.1-sonar-large-128k-chat" => Self::Llama31SonarLarge128KChat,
            "llama-3.1-8b-instruct" => Self::Llama318BInstruct,
            "llama-3.1-70b-instruct" => Self::Llama3170BInstruct,
            _ => Err(AiInterfaceError::DBError(format!(
                "Perplexity unknown model {}",
                s
            )))?,
        })
    }
}
