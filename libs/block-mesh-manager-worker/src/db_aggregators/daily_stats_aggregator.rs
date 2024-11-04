use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DailyStatMessage;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;
use uuid::Uuid;

pub fn daily_stats_create_bulk_query(calls: HashMap<Uuid, f64>) -> String {
    let values: Vec<String> = calls
        .iter()
        .map(|(id, value)| format!("('{}'::uuid, {})", id, value))
        .collect();
    let value_str = values.join(",");

    format!(
        r#"
WITH updates (id, value) AS (
VALUES {}
)
-- Update statement using the CTE
UPDATE daily_stats
SET
    uptime = uptime + updates.value
FROM updates
WHERE daily_stats.id = updates.id;
    "#,
        value_str
    )
}

#[tracing::instrument(name = "daily_stats_aggregator", skip_all, err)]
pub async fn daily_stats_aggregator(
    joiner_tx: Sender<JoinHandle<()>>,
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    loop {
        match rx.recv().await {
            Ok(message) => {
                if let Ok(message) = serde_json::from_value::<DailyStatMessage>(message) {
                    calls.insert(message.id, message.uptime);
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        let calls_clone = calls.clone();
                        let poll_clone = pool.clone();
                        let handle = tokio::spawn(async move {
                            tracing::info!("daily_stats_aggregator starting txn");
                            if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                let query = daily_stats_create_bulk_query(calls_clone);
                                let _ = sqlx::query(&query).execute(&mut *transaction).await;
                                let _ = commit_txn(transaction).await;
                            }
                        });
                        let _ = joiner_tx.send_async(handle).await;
                        count = 0;
                        calls.clear();
                        tracing::info!("daily_stats_aggregator finished txn");
                    }
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("daily_stats_aggregator error recv: {:?}", e);
                    return Err(anyhow!("daily_stats_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("daily_stats_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
