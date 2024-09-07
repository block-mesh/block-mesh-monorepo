use crate::db_calls::get_or_create_analytics::get_or_create_analytics;
use block_mesh_common::interfaces::db_messages::AnalyticsMessage;
use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::Receiver;

pub async fn analytics_aggregator(
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv().await {
        if let Ok(message) = serde_json::from_value::<AnalyticsMessage>(message) {
            calls.insert(message.user_id, message.clone());
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
                        let _ = get_or_create_analytics(
                            &mut transaction,
                            pair.0,
                            &pair.1.depin_aggregator,
                            &pair.1.device_type,
                        )
                        .await;
                    }
                    let _ = transaction.commit().await;
                }
            }
        }
    }
    Ok(())
}
