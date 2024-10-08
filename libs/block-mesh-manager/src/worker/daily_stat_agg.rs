use crate::database::daily_stat::update_daily_stat_uptime_bulk::update_daily_stat_uptime_bulk;
use crate::startup::application::AppState;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::db_messages::DailyStatMessage;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use flume::Receiver;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "daily_stat_agg", skip_all)]
pub async fn daily_stat_agg(
    pool: PgPool,
    rx: Receiver<DailyStatMessage>,
    state: Arc<AppState>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<Uuid, f64> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    while let Ok(message) = rx.recv_async().await {
        let flag = state
            .flags
            .get("send_to_worker")
            .unwrap_or(&FlagValue::Boolean(false));
        let flag: bool =
            <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
        if flag {
            let _ = notify_worker(&pool, message.clone()).await;
        } else {
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
    }
    Ok(())
}

#[tracing::instrument(name = "daily_stat_submit_to_db", skip_all)]
pub async fn daily_stat_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<Uuid, f64>,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(pool).await?;
    update_daily_stat_uptime_bulk(&mut transaction, calls).await?;
    commit_txn(transaction).await?;
    Ok(())
}
