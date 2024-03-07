use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Sender;
use uuid::Uuid;
pub type TokenManagerHashMap = FxHashMap<Uuid, TokenDetails>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDetails {
    token: Uuid,
    bandwidth_allowance: u64,
    bandwidth_used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMessage {
    pub upload: u64,
    pub download: u64,
    pub token: Uuid,
}

#[tracing::instrument(name = "update_token_manager")]
pub async fn update_token_manager(msg: &ChannelMessage, token_manager: &mut TokenManagerHashMap) {
    if !token_manager.contains_key(&msg.token) {
        token_manager.insert(
            msg.token,
            TokenDetails {
                token: msg.token,
                bandwidth_allowance: 0,
                bandwidth_used: 0,
            },
        );
    } else {
        let details = token_manager.get_mut(&msg.token).unwrap();
        details.bandwidth_used += msg.download;
        details.bandwidth_used += msg.upload;
    }

    tracing::info!(">>> token manager: {:?}", token_manager.get(&msg.token));
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
