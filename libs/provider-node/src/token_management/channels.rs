use anchor_lang::prelude::Pubkey;
use block_mesh_solana_client::manager::FullRouteHeader;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;

pub type TokenManagerHashMap = FxHashMap<Pubkey, TokenDetails>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDetails {
    pub bandwidth_allowance: u64,
    pub bandwidth_used: u64,
    pub nonce: String,
    pub signature: String,
    pub pubkey: Pubkey,
    pub api_token: Pubkey,
}

impl TokenDetails {
    pub fn is_valid(&self, solana_manager_auth: &FullRouteHeader) -> bool {
        self.nonce == solana_manager_auth.client_signature.nonce
            && self.signature == solana_manager_auth.client_signature.signature
            && self.pubkey == solana_manager_auth.client_signature.pubkey
            && self.api_token == solana_manager_auth.api_token
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    pub upload: u64,
    pub download: u64,
    pub api_token: Pubkey,
}

#[tracing::instrument(name = "update_token_manager")]
pub async fn update_token_manager(
    msg: &ChannelMessage,
    token_manager: Arc<RwLock<TokenManagerHashMap>>,
) {
    let mut token_manager = token_manager.write().await;
    match token_manager.get_mut(&msg.api_token) {
        Some(details) => {
            details.bandwidth_used += msg.download;
            details.bandwidth_used += msg.upload;
        }
        None => {
            tracing::error!("api_token not found: {:?}", msg.api_token);
        }
    }
}

#[tracing::instrument(name = "send_message")]
pub fn send_message(tx: &Sender<ChannelMessage>, msg: ChannelMessage) {
    match tx.send(msg) {
        Ok(_) => {}
        Err(e) => {
            tracing::warn!("failed to send channel message: {}", e);
        }
    }
}
