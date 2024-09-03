use crate::database::aggregate::update_aggregate_bulk::update_aggregate_bulk;
use chrono::Utc;
use flume::Receiver;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AggregateMessage {
    pub id: Uuid,
    pub value: Value,
}

pub async fn aggregate_agg(
    pool: PgPool,
    rx: Receiver<AggregateMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGGREGATE_AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<Uuid, Value> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv_async().await {
        calls.insert(message.id, message.value);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            let _ = aggregate_submit_to_db(&pool, &mut calls).await;
            count = 0;
            calls.clear();
        }
    }
    Ok(())
}

#[tracing::instrument(name = "users_ips_submit_to_db", skip(pool, calls), ret, err)]
pub async fn aggregate_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<Uuid, Value>,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    update_aggregate_bulk(&mut transaction, calls).await?;
    transaction.commit().await?;
    Ok(())
}
