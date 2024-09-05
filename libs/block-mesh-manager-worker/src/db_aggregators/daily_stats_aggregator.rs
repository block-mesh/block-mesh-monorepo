use crate::db_calls::increment_uptime::increment_uptime;
use block_mesh_common::interfaces::db_messages::DailyStatMessage;
use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::Receiver;

pub async fn daily_stats_aggregator(
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv().await {
        tracing::info!("(1) daily_stats_aggregator incoming message {:#?}", message);
        if let Ok(message) = serde_json::from_value::<DailyStatMessage>(message) {
            tracing::info!("(2) daily_stats_aggregator incoming message {:#?}", message);
            calls.insert(message.id, message.uptime);
            count += 1;
            let now = Utc::now();
            let diff = now - prev;
            let run = diff.num_seconds() > time_limit || count >= agg_size;
            prev = Utc::now();
            if run {
                count = 0;
                calls.clear();
                if let Ok(mut transaction) = pool.begin().await {
                    for pair in calls.iter() {
                        let _ = increment_uptime(&mut transaction, pair.0, *pair.1).await;
                    }
                    let _ = transaction.commit().await;
                }
            }
        }
    }
    Ok(())
}
