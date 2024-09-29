use crate::websocket::manager::broadcaster::Broadcaster;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use std::env;
use std::hash::Hash;
use std::time::Duration;

pub async fn ws_keep_alive<T: Hash + Eq + Clone>(
    broadcaster: Broadcaster<T>,
) -> Result<(), anyhow::Error> {
    loop {
        let _ = broadcaster.broadcast(WsServerMessage::Ping);
        tokio::time::sleep(Duration::from_millis(
            env::var("WS_KEEP_ALIVE")
                .ok()
                .and_then(|var| var.parse().ok())
                .unwrap_or(15000),
        ))
        .await;
    }
}
