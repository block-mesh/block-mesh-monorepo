use block_mesh_common::interfaces::ws_api::{WsMessage, WsMessageTypes};
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::time::sleep;
use uuid::Uuid;

pub async fn ws_worker_rx(
    _pool: PgPool,
    mut rx: Receiver<WsMessage>,
    _tx: Sender<WsMessage>,
) -> Result<(), anyhow::Error> {
    while let Ok(msg) = rx.recv().await {
        tracing::info!("RX msg => {:#?}", msg);
    }
    Ok(())
}

pub async fn ws_worker_tx(
    _pool: PgPool,
    _rx: Receiver<WsMessage>,
    tx: Sender<WsMessage>,
) -> Result<(), anyhow::Error> {
    loop {
        tracing::info!("Waking up");
        let _ = tx.send(WsMessage {
            message_id: Uuid::new_v4(),
            email: None,
            device: None,
            message: WsMessageTypes::SendUptimeFromServer,
        });
        sleep(Duration::from_secs(15)).await;
    }
    Ok(())
}
