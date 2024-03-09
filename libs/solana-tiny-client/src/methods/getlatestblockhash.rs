use crate::general::commitment::CommitmentParams;
use crate::general::context::Context;
use crate::general::jsonrpc::Jsonrpc;
use crate::general::methods::Methods;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestBlockhashInput {
    pub(crate) id: u64,
    pub(crate) jsonrpc: Jsonrpc,
    pub(crate) method: Methods,
    pub(crate) params: [CommitmentParams; 1],
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct GetLatestBlockhashOutputValue {
    blockhash: String,
    lastValidBlockHeight: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestBlockhashOutputResult {
    context: Context,
    value: GetLatestBlockhashOutputValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestBlockhashOutput {
    id: u64,
    jsonrpc: Jsonrpc,
    result: GetLatestBlockhashOutputResult,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::general::commitment::Commitment;
    use crate::PUBLIC_URLS;
    use reqwest::header::CONTENT_TYPE;

    #[tokio::test]
    async fn test_get_latest_blockhash() {
        let body = GetLatestBlockhashInput {
            id: 1,
            jsonrpc: Jsonrpc::Jsonrpc,
            method: Methods::GetLatestBlockhash,
            params: [CommitmentParams {
                commitment: Commitment::Processed,
            }],
        };

        let body_json = serde_json::to_string(&body)
            .map_err(|e| println!("GetLatestBlockHash (0) => {:?}", e))
            .unwrap();

        println!("body_json: {:?}", body_json);

        let dynamic_json: serde_json::Value = reqwest::Client::new()
            .post(PUBLIC_URLS[2])
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        println!("dynamic_json: {:?}", dynamic_json);

        let response = reqwest::Client::new()
            .post(PUBLIC_URLS[2])
            .header(CONTENT_TYPE, "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| println!("GetLatestBlockHash (1) => {:?}", e))
            .unwrap();

        println!("GetLatestBlockHash (*) => {:?}", response);

        let response = response
            .json::<GetLatestBlockhashOutput>()
            .await
            .map_err(|e| {
                println!("GetLatestBlockHash (2) => {:?}", e);
            })
            .unwrap();

        println!("GetLatestBlockHash (3) => {:?}", response);
    }
}
