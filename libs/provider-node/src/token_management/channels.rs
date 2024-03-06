use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, Mutex, OnceCell};
use uuid::Uuid;

pub static TX: OnceCell<Sender<ChannelMessage>> = OnceCell::const_new();
pub static TOKEN_MANAGER: OnceCell<Mutex<FxHashMap<Uuid, TokenDetails>>> = OnceCell::const_new();

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

pub async fn update_token_manager(msg: &ChannelMessage) {
    let mut token_manager = match TOKEN_MANAGER.get() {
        Some(token_manager) => token_manager.lock().await,
        None => {
            tracing::warn!("Token manager not initialized");
            return;
        }
    };
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

    println!(">>> token manager: {:?}", token_manager.get(&msg.token));
}

pub fn init_channels() -> Receiver<ChannelMessage> {
    let (tx, rx1) = broadcast::channel::<ChannelMessage>(16);
    TX.set(tx).unwrap();
    rx1
}

pub fn init_token_manager() {
    TOKEN_MANAGER.set(FxHashMap::default().into()).unwrap()
}

pub fn send_message(msg: ChannelMessage) {
    match TX.get() {
        Some(tx) => match tx.send(msg) {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("failed to send channel message: {}", e);
            }
        },
        None => {
            tracing::warn!("TX not initialized");
        }
    }
}
