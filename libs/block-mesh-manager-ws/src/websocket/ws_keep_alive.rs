use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use std::env;
use std::time::Duration;

pub async fn ws_keep_alive(broadcaster: Broadcaster) -> Result<(), anyhow::Error> {
    let sleep = env::var("WS_KEEP_ALIVE")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(15000);
    loop {
        let _ = broadcaster.broadcast(WsServerMessage::Ping);
        tokio::time::sleep(Duration::from_millis(sleep)).await;
    }
}
