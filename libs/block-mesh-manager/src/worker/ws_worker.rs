use block_mesh_common::interfaces::ws_api::{WsMessage, WsMessageTypes};
use futures::future::join_all;
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time::sleep;
use uuid::Uuid;

pub async fn ws_worker_rx(
    pool: PgPool,
    mut rx: Receiver<WsMessage>,
    tx: Sender<WsMessage>,
) -> Result<(), anyhow::Error> {
    while let Ok(msg) = rx.recv().await {
        tracing::info!("RX msg => {:#?}", msg);
    }
    Ok(())
}

pub async fn ws_worker_tx(
    pool: PgPool,
    mut rx: Receiver<WsMessage>,
    tx: Sender<WsMessage>,
) -> Result<(), anyhow::Error> {
    loop {
        tracing::info!("Waking up");
        let _ = tx.send(WsMessage {
            message_id: Uuid::new_v4(),
            email: None,
            device: None,
            message: WsMessageTypes::SendUptimeToNode,
        });
        sleep(Duration::from_secs(5)).await;
    }
    Ok(())
}
