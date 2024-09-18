use serde_json::Value;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;

pub async fn send_to_rx(payload: Value, tx: Arc<Sender<Value>>) {
    let _ = tx.send(payload);
}
