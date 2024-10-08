use crate::database::analytics::inserting_client_analytics_bulk::inserting_client_analytics_bulk;
use crate::database::notify::notify_worker::notify_worker;
use crate::startup::application::AppState;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::db_messages::AnalyticsMessage;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use flume::Receiver;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "analytics_agg", skip_all)]
pub async fn analytics_agg(
    pool: PgPool,
    rx: Receiver<AnalyticsMessage>,
    state: Arc<AppState>,
) -> Result<(), anyhow::Error> {
    let agg_size = env::var("AGG_SIZE")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut calls: HashMap<Uuid, AnalyticsMessage> = HashMap::new();
    let mut count = 0;
    let mut prev = Utc::now();
    let flag = state
        .flags
        .get("send_to_worker")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    while let Ok(query) = rx.recv_async().await {
        if flag {
            let _ = notify_worker(&pool, query.clone()).await;
        } else {
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
    }
    Ok(())
}

#[tracing::instrument(name = "analytics_agg_submit_to_db", skip_all)]
pub async fn analytics_agg_submit_to_db(
    pool: &PgPool,
    calls: &mut HashMap<Uuid, AnalyticsMessage>,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(pool).await?;
    inserting_client_analytics_bulk(&mut transaction, calls).await?;
    commit_txn(transaction).await?;
    Ok(())
}
