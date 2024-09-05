use crate::database::analytics::inserting_client_analytics_bulk::inserting_client_analytics_bulk;
use block_mesh_common::interfaces::db_messages::AnalyticsMessage;
use chrono::Utc;
use flume::Receiver;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

pub async fn analytics_agg(
    pool: PgPool,
    rx: Receiver<AnalyticsMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("ANALYTICS_AGG_AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<Uuid, AnalyticsMessage> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(query) = rx.recv_async().await {
        calls.insert(query.user_id, query.clone());
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 10 || count >= agg_size;
        prev = Utc::now();
        if run {
            let _ = analytics_agg_submit_to_db(&pool, &mut calls).await;
            count = 0;
            calls.clear();
        }
    }
    Ok(())
}

#[tracing::instrument(name = "analytics_agg_submit_to_db", skip(pool, calls), ret, err)]
pub async fn analytics_agg_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<Uuid, AnalyticsMessage>,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    inserting_client_analytics_bulk(&mut transaction, calls).await?;
    transaction.commit().await?;
    Ok(())
}
