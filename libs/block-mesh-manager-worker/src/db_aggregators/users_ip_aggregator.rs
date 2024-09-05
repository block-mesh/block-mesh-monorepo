use crate::call_backs::touch_users_ip::touch_users_ip;
use block_mesh_common::interfaces::db_messages::UsersIpMessage;
use chrono::Utc;
use flume::Receiver;
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn users_ip_aggregator(
    pool: PgPool,
    rx: Receiver<Value>,
    agg_size: i32,
    time_limit: i64,
) -> Result<(), anyhow::Error> {
    let mut calls: HashMap<_, _> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv_async().await {
        tracing::info!("1 aggregator incoming message {:#?}", message);
        if let Ok(m) = serde_json::from_value::<UsersIpMessage>(message) {
            tracing::info!("2 aggregator incoming message {:#?}", m);
            calls.insert(m.id, m.ip);
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
                        let _ = touch_users_ip(&mut transaction, pair.0, pair.1).await;
                    }
                    let _ = transaction.commit().await;
                }
            }
        }
    }
    Ok(())
}
