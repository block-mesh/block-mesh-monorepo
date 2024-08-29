use crate::database::aggregate::update_aggregate::update_aggregate_bulk;
use crate::database::daily_stat::increment_uptime::update_daily_stat_uptime_bulk;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateBulkMessage {
    pub id: Uuid,
    pub value: Value,
    pub table: Table,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Table {
    DailyStat,
    Aggregate,
}

pub async fn db_agg(
    pool: PgPool,
    mut rx: tokio::sync::mpsc::Receiver<UpdateBulkMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGG_SIZE")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);
    let mut queries: Vec<String> = Vec::with_capacity(agg_size * 3);
    let mut calls: HashMap<Uuid, Value> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Some(query) = rx.recv().await {
        calls.insert(query.id, query.value);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            if let Ok(mut transaction) = pool.begin().await {
                match query.table {
                    Table::Aggregate => {
                        match update_aggregate_bulk(&mut transaction, &mut calls).await {
                            Ok(_) => {}
                            Err(e) => tracing::error!("ERROR update_aggregate_bulk {}", e),
                        }
                    }
                    Table::DailyStat => {
                        match update_daily_stat_uptime_bulk(&mut transaction, &mut calls).await {
                            Ok(_) => {}
                            Err(e) => tracing::error!("ERROR update_daily_stat_uptime_bulk {}", e),
                        }
                    }
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
