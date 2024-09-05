use flume::Sender;
use serde_json::Value;
use std::sync::Arc;

pub async fn send_to_rx(payload: Value, tx: Arc<Sender<Value>>) {
    tracing::info!("Payload received: {:#?}", payload);
    let _ = tx.send_async(payload).await;
}
