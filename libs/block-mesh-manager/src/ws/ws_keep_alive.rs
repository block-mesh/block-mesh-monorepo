use crate::ws::broadcaster::Broadcaster;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use std::env;
use std::time::Duration;

pub async fn ws_keep_alive(broadcaster: Broadcaster) -> Result<(), anyhow::Error> {
    loop {
        let _ = broadcaster.broadcast(WsServerMessage::Ping);
        tokio::time::sleep(Duration::from_millis(
            env::var("WS_KEEP_ALIVE")
                .unwrap_or("15000".to_string())
                .parse()
                .unwrap_or(15000),
        ))
        .await;
    }
}
