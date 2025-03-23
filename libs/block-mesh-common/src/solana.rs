use anyhow::anyhow;
use solana_sdk::signature::Keypair;
use std::env;

pub fn get_keypair() -> anyhow::Result<Keypair> {
    let data: serde_json::Value =
        serde_json::from_str(&env::var("EXT_KEYPAIR")?).map_err(|e| anyhow!(e.to_string()))?;
    let key_bytes: Vec<u8> = serde_json::from_value(data).map_err(|e| anyhow!(e.to_string()))?;
    Keypair::from_bytes(&key_bytes).map_err(|e| anyhow!(e.to_string()))
}
