use anyhow::anyhow;
use chrono::Utc;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::env;

pub fn get_keypair() -> anyhow::Result<Keypair> {
    let data: serde_json::Value =
        serde_json::from_str(&env::var("EXT_KEYPAIR")?).map_err(|e| anyhow!(e.to_string()))?;
    let key_bytes: Vec<u8> = serde_json::from_value(data).map_err(|e| anyhow!(e.to_string()))?;
    Keypair::from_bytes(&key_bytes).map_err(|e| anyhow!(e.to_string()))
}

pub async fn get_block_time() -> i64 {
    let timestamp = Utc::now().timestamp();
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new(rpc_url);
    let slot = client.get_slot().unwrap_or(370679187);
    client.get_block_time(slot).unwrap_or(timestamp)
}
