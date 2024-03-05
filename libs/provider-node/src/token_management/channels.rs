use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, OnceCell};
use uuid::Uuid;

pub static TX: OnceCell<Sender<ChannelMessage>> = OnceCell::const_new();

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

pub fn init_channels() -> Receiver<ChannelMessage> {
    let (tx, mut rx1) = broadcast::channel::<ChannelMessage>(16);
    TX.set(tx).unwrap();
    rx1
}
