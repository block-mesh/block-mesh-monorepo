use crate::db_calls::get_or_create_analytics::get_or_create_analytics;
use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::AnalyticsMessage;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;

#[tracing::instrument(name = "analytics_aggregator", skip_all, err)]
pub async fn analytics_aggregator(
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
                if let Ok(message) = serde_json::from_value::<AnalyticsMessage>(message) {
                    calls.insert(message.user_id, message.clone());
                    count += 1;
                    let now = Utc::now();
                    let diff = now - prev;
                    let run = diff.num_seconds() > time_limit || count >= agg_size;
                    prev = Utc::now();
                    if run {
                        tracing::info!("analytics_aggregator starting txn");
                        if let Ok(mut transaction) = create_txn(&pool).await {
                            for pair in calls.iter() {
                                let _ = get_or_create_analytics(
                                    &mut transaction,
                                    pair.0,
                                    &pair.1.depin_aggregator,
                                    &pair.1.device_type,
                                    &pair.1.version,
                                )
                                .await;
                            }
                            let _ = commit_txn(transaction).await;
                            count = 0;
                            calls.clear();
                            tracing::info!("analytics_aggregator finished txn");
                        }
                    }
                }
            }
            Err(e) => match e {
                RecvError::Closed => {
                    tracing::error!("analytics_aggregator error recv: {:?}", e);
                    return Err(anyhow!("analytics_aggregator error recv: {:?}", e));
                }
                RecvError::Lagged(_) => {
                    tracing::error!("analytics_aggregator error recv: {:?}", e);
                }
            },
        }
    }
}
