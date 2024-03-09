use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub enum Methods {
    #[serde(rename = "getLatestBlockhash")]
    GetLatestBlockhash,
}

impl Display for Methods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Methods::GetLatestBlockhash => "getLatestBlockhash".to_string(),
        };
        write!(f, "{}", str)
    }
}
