use crate::db_calls::touch_users_ip::touch_users_ip;
use anyhow::anyhow;
use block_mesh_common::interfaces::db_messages::UsersIpMessage;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use tokio::sync::broadcast::Receiver;

#[tracing::instrument(name = "users_ip_aggregator", skip_all, err)]
pub async fn users_ip_aggregator(
    pool: PgPool,
    mut rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv().await {
        if let Ok(message) = serde_json::from_value::<UsersIpMessage>(message) {
            calls.insert(message.id, message.ip);
            count += 1;
            let now = Utc::now();
            let diff = now - prev;
            let run = diff.num_seconds() > time_limit || count >= agg_size;
            prev = Utc::now();
            if run {
                tracing::info!("users_ip_aggregator starting txn");
                if let Ok(mut transaction) = create_txn(&pool).await {
                    for pair in calls.iter() {
                        let _ = touch_users_ip(&mut transaction, pair.0, pair.1).await;
                    }
                    let _ = commit_txn(transaction).await;
                }
                count = 0;
                calls.clear();
                tracing::info!("users_ip_aggregator finished txn");
            }
        }
    }
    Err(anyhow!("users_ip_aggregator terminated"))
}
