use crate::database::aggregate::update_aggregate::inserting_client_analytics_bulk;
use block_mesh_common::constants::DeviceType;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AnalyticsMessage {
    pub user_id: Uuid,
    pub depin_aggregator: String,
    pub device_type: DeviceType,
}

pub async fn analytics_agg(
    pool: PgPool,
    mut rx: tokio::sync::mpsc::Receiver<AnalyticsMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGG_SIZE")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);
    let mut queries: Vec<String> = Vec::with_capacity(agg_size * 3);
    let mut calls: HashMap<Uuid, AnalyticsMessage> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Some(query) = rx.recv().await {
        calls.insert(query.user_id, query.clone());
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            if let Ok(mut transaction) = pool.begin().await {
                match inserting_client_analytics_bulk(&mut transaction, &mut calls).await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("ERROR inserting_client_analytics_bulk {}", e),
                }
                match transaction.commit().await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("ERROR update_aggregate_bulk commit {}", e),
                }
                calls.clear();
                queries.clear();
                count = 0;
            }
        }
    }
    Ok(())
}
