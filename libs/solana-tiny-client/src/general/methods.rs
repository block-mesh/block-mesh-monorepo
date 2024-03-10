use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Methods {
    #[serde(rename = "getLatestBlockhash")]
    GetLatestBlockhash,
    #[serde(rename = "sendTransaction")]
    SendTransaction,
}
