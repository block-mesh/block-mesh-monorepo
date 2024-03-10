use crate::general::commitment::{Commitment, CommitmentParams};
use crate::general::context::Context;
use crate::general::jsonrpc::Jsonrpc;
use crate::general::methods::Methods;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLatestBlockhashInput {
    id: u64,
    jsonrpc: Jsonrpc,
    method: Methods,
    params: [CommitmentParams; 1],
}

impl GetLatestBlockhashInput {
    pub fn new(commitment: Commitment) -> Self {
        GetLatestBlockhashInput {
            id: 1,
            jsonrpc: Jsonrpc::Jsonrpc,
            method: Methods::GetLatestBlockhash,
            params: [CommitmentParams { commitment }],
        }
    }
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
    use crate::client::rpc_client::RpcClient;
    use crate::general::commitment::Commitment;
    use crate::PUBLIC_URLS;

    #[tokio::test]
    async fn test_get_latest_blockhash() {
        let response = RpcClient::new(PUBLIC_URLS[2].to_string(), Commitment::Confirmed)
            .get_latest_blockhash(None, None)
            .await
            .unwrap();
        assert!(response.result.value.blockhash.len() > 0);
        assert!(response.result.value.lastValidBlockHeight > 0);
    }
}
