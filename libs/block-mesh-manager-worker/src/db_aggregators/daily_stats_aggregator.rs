use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::DBMessage;
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

#[tracing::instrument(name = "daily_stats_create_bulk_query", skip_all)]
pub fn daily_stats_create_bulk_query(calls: HashMap<Uuid, f64>) -> String {
    let values: Vec<String> = calls
        .iter()
        .map(|(id, value)| format!("('{}'::uuid, {})", id, value))
        .collect();
    let value_str = values.join(",");
    let lock_values: Vec<String> = calls.keys().map(|id| format!("'{}'::uuid", id)).collect();
    let lock_str = lock_values.join(",");
    format!(
        r#"
WITH
updates (id, value) AS (VALUES {value_str}),
locked_rows (id) AS (SELECT id FROM daily_stats WHERE id in ({lock_str}) FOR UPDATE SKIP LOCKED)
UPDATE daily_stats
SET uptime = uptime + updates.value
FROM updates
JOIN locked_rows ON locked_rows.id = updates.id
WHERE daily_stats.id = updates.id;
    "#
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
                if let Ok(DBMessage::DailyStatMessage(message)) =
                    serde_json::from_value::<DBMessage>(message)
                {
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
                            tracing::info!("daily_stats_create_bulk_query starting txn");
                            if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                let query = daily_stats_create_bulk_query(calls_clone);
                                let r = sqlx::query(&query)
                                    .execute(&mut *transaction)
                                    .await
                                    .map_err(|e| {
                                        tracing::error!(
                                            "daily_stats_create_bulk_query failed to execute query size: {} , with error {:?}",
                                            count,
                                            e
                                        );
                                    });
                                if let Ok(r) = r {
                                    tracing::info!(
                                        "daily_stats_create_bulk_query rows_affected : {}",
                                        r.rows_affected()
                                    );
                                }
                                let _ = commit_txn(transaction).await;
                                tracing::info!("daily_stats_create_bulk_query finished txn");
                            }
                        });
                        let _ = joiner_tx.send_async(handle).await;
                        count = 0;
                        calls.clear();
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
