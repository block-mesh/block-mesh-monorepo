use crate::database::daily_stat::update_daily_stat_uptime_bulk::update_daily_stat_uptime_bulk;
use block_mesh_common::interfaces::db_messages::DailyStatMessage;
use chrono::Utc;
use flume::Receiver;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

pub async fn daily_stat_agg(
    pool: PgPool,
    rx: Receiver<DailyStatMessage>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("DAILY_STAT_AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<Uuid, f64> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv_async().await {
        calls.insert(message.id, message.uptime);
        count += 1;
        let now = Utc::now();
        let diff = now - prev;
        let run = diff.num_seconds() > 5 || count >= agg_size;
        prev = Utc::now();
        if run {
            let _ = daily_stat_submit_to_db(&pool, &mut calls).await;
            count = 0;
            calls.clear();
        }
    }
    Ok(())
}

#[tracing::instrument(name = "daily_stat_submit_to_db", skip(pool, calls), ret, err)]
pub async fn daily_stat_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<Uuid, f64>,
) -> anyhow::Result<()> {
    let mut transaction = pool.begin().await?;
    update_daily_stat_uptime_bulk(&mut transaction, calls).await?;
    transaction.commit().await?;
    Ok(())
}
