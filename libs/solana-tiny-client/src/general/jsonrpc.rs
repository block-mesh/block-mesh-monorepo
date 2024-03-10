use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Jsonrpc {
    #[serde(rename = "2.0")]
    Jsonrpc,
}
