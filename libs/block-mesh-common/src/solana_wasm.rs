use crate::constants::DeviceType;
use crate::feature_flag_client::get_flag_value;
use crate::reqwest::http_client;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct GetSlotRequest {
    jsonrpc: String,
    id: u64,
    method: String,
}

impl Default for GetSlotRequest {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getSlot".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GetSlotResponse {
    pub jsonrpc: String,
    pub result: i64,
    pub id: i64,
}

#[derive(Serialize, Deserialize)]
struct GetBlockTimeRequest {
    pub jsonrpc: String,
    pub id: i64,
    pub method: String,
    pub params: Vec<i64>,
}

impl GetBlockTimeRequest {
    pub fn new(slot: i64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "getBlockTime".to_string(),
            params: vec![slot],
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GetBlockTimeResponse {
    pub jsonrpc: String,
    pub result: i64,
    pub id: i64,
}

pub async fn get_slot() -> anyhow::Result<i64> {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = reqwest::Client::new();
    let resp: GetSlotResponse = client
        .post(rpc_url)
        .json(&GetSlotRequest::default())
        .send()
        .await?
        .json()
        .await?;
    Ok(resp.result)
}

pub async fn _get_block_time(slot: i64) -> anyhow::Result<i64> {
    let rpc_url =
        env::var("SOLANA_RPC").unwrap_or("https://api.mainnet-beta.solana.com".to_string());
    let client = reqwest::Client::new();
    let resp: GetBlockTimeResponse = client
        .post(rpc_url)
        .json(&GetBlockTimeRequest::new(slot))
        .send()
        .await?
        .json()
        .await?;
    Ok(resp.result)
}

pub async fn get_block_time() -> i64 {
    let timestamp = Utc::now().timestamp();
    if let Ok(slot) = get_slot().await {
        if let Ok(block) = _get_block_time(slot).await {
            return block;
        }
    }
    timestamp
}

pub async fn get_block_time_from_feature_flags() -> i64 {
    let timestamp = Utc::now().timestamp();
    let client = http_client(DeviceType::Extension);
    let block_time = get_flag_value("block_time", &client, DeviceType::Extension).await;
    match block_time.unwrap_or(None) {
        Some(i) => {
            if i.is_i64() {
                i.as_i64().unwrap()
            } else {
                timestamp
            }
        }
        None => timestamp,
    }
}

pub async fn get_minimal_version_from_feature_flags() -> String {
    let minimal_version = "0.0.515".to_string();
    let client = http_client(DeviceType::Extension);
    let block_time = get_flag_value("minimal_version", &client, DeviceType::Extension).await;
    match block_time.unwrap_or(None) {
        Some(i) => {
            if i.is_string() {
                i.as_str()
                    .unwrap_or(&minimal_version)
                    .trim_matches('"')
                    .to_string()
            } else {
                minimal_version
            }
        }
        None => minimal_version,
    }
}
