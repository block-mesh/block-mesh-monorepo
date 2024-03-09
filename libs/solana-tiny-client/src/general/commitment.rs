use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Debug, Serialize, Deserialize)]
pub enum Commitment {
    Finalized,
    Confirmed,
    Processed,
}

impl Display for Commitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Commitment::Finalized => "finalized".to_string(),
            Commitment::Confirmed => "confirmed".to_string(),
            Commitment::Processed => "processed".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitmentParams {
    pub commitment: Commitment,
}
