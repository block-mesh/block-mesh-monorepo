use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::AggregateMessage;
use block_mesh_manager_database_domain::domain::update_aggregate::update_aggregate;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use flume::Sender;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::task::JoinHandle;

#[tracing::instrument(name = "aggregates_aggregator", skip_all, err)]
pub async fn aggregates_aggregator(
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
                if let Ok(message) = serde_json::from_value::<AggregateMessage>(message) {
                    calls.insert(message.id, message.value);
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        let calls_clone = calls.clone();
                        let poll_clone = pool.clone();
                        let handle = tokio::spawn(async move {
                            tracing::info!("aggregates_aggregator starting txn");
                            if let Ok(mut transaction) = create_txn(&poll_clone).await {
                                for pair in calls_clone.iter() {
                                    let _ =
                                        update_aggregate(&mut transaction, pair.0, pair.1).await;
                                }
                                let _ = commit_txn(transaction).await;
                            }
                            tracing::info!("aggregates_aggregator finished txn");
                        });
                        let _ = joiner_tx.send_async(handle).await;
                        count = 0;
                        calls.clear();
                    }
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("aggregates_aggregator error recv: {:?}", e);
                    return Err(anyhow!("aggregates_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("aggregates_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
