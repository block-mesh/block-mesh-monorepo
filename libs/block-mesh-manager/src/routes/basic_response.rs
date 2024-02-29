use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq)]
pub enum ResponseStatus {
    Success,
    #[default]
    Failure,
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ResponseStatus::Success => "Success".to_string(),
            ResponseStatus::Failure => "Failure".to_string(),
        };
        write!(f, "{}", str)
    }
}
