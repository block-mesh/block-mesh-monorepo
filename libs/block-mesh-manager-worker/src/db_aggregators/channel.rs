use flume::Receiver;
use serde_json::Value;
use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub async fn channel_aggregator(
    _pool: Pool<Postgres>,
    rx: Receiver<Value>,
) -> Result<(), anyhow::Error> {
    while let Ok(message) = rx.recv_async().await {
        tracing::info!("channel_aggregator message {:#?}", message);
    }
    Ok(())
}
