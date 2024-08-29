use crate::database::aggregate::update_aggregate::update_aggregate_bulk;
use crate::database::daily_stat::increment_uptime::{
    update_daily_stat_uptime_bulk, update_users_ip_bulk,
};
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
    UserIp,
}

pub async fn db_agg(
    pool: PgPool,
    mut rx: tokio::sync::mpsc::Receiver<UpdateBulkMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGG_SIZE")
        .unwrap_or("100".to_string())
        .parse()
        .unwrap_or(100);
    let mut agg_calls: HashMap<Uuid, Value> = HashMap::new();
    let mut daily_calls: HashMap<Uuid, Value> = HashMap::new();
    let mut uptime_calls: HashMap<Uuid, Value> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Some(query) = rx.recv().await {
        match query.table {
            Table::Aggregate => agg_calls.insert(query.id, query.value),
            Table::DailyStat => daily_calls.insert(query.id, query.value),
            Table::UserIp => uptime_calls.insert(query.id, query.value),
        };
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            if let Ok(mut transaction) = pool.begin().await {
                match query.table {
                    Table::Aggregate => {
                        match update_aggregate_bulk(&mut transaction, &mut agg_calls).await {
                            Ok(_) => {}
                            Err(e) => tracing::error!("ERROR update_aggregate_bulk {}", e),
                        }
                    }
                    Table::DailyStat => {
                        match update_daily_stat_uptime_bulk(&mut transaction, &mut daily_calls)
                            .await
                        {
                            Ok(_) => {}
                            Err(e) => tracing::error!("ERROR update_daily_stat_uptime_bulk {}", e),
                        }
                    }
                    Table::UserIp => {
                        match update_users_ip_bulk(&mut transaction, &mut uptime_calls).await {
                            Ok(_) => {}
                            Err(e) => tracing::error!("ERROR update_users_ip_bulk {}", e),
                        }
                    }
                }
                match transaction.commit().await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("ERROR update_aggregate_bulk commit {}", e),
                }
                agg_calls.clear();
                daily_calls.clear();
                uptime_calls.clear();
                count = 0;
            }
        }
    }
    Ok(())
}
