use crate::data_sink::{DataSinkClickHouse, CLICKHOUSE_TABLE_NAME};
use chrono::Utc;
use clickhouse::Client;
use flume::Receiver;
use flume::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "collect_writes_for_clickhouse", skip_all)]
pub async fn collect_writes_for_clickhouse(
    clickhouse_client: Arc<Client>,
    joiner_tx: Sender<JoinHandle<()>>,
    rx: Receiver<DataSinkClickHouse>,
    agg_size: i32,
    time_limit: i64,
) -> anyhow::Result<()> {
    let mut cache: HashMap<(String, String), DataSinkClickHouse> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(msg) = rx.recv_async().await {
        let key = (msg.origin.clone(), msg.origin_id.clone());
        cache.insert(key, msg);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > time_limit || count >= agg_size;
        prev = Utc::now();
        if run {
            let cache_clone = cache.clone();
            let clickhouse_client_clone = clickhouse_client.clone();
            let handle = tokio::spawn(async move {
                tracing::info!("collect_writes_for_clickhouse starting txn");
                if let Ok(mut insert) = clickhouse_client_clone.insert(CLICKHOUSE_TABLE_NAME) {
                    for entry in cache_clone.into_iter() {
                        let _ = insert.write(&entry.1).await;
                    }
                    let _ = insert
                        .end()
                        .await
                        .map_err(|e| tracing::error!("Error writing to clickhouse {}", e));
                    tracing::info!("collect_writes_for_clickhouse done txn");
                }
            });
            let _ = joiner_tx.send_async(handle).await;
            count = 0;
            cache.clear();
        }
    }
    Ok(())
}

#[tracing::instrument(name = "joiner_loop", skip_all)]
pub async fn joiner_loop(rx: Receiver<JoinHandle<()>>) -> Result<(), anyhow::Error> {
    while let Ok(handle) = rx.recv_async().await {
        let _ = handle.await;
    }
    Ok(())
}
