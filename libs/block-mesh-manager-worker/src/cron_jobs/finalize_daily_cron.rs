use crate::db_calls::bulk_finalize::bulk_finalize;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;

#[tracing::instrument(name = "finalize_daily_cron", skip_all)]
pub async fn finalize_daily_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    let finalize_sleep = env::var("FINALIZE_SLEEP_SECS")
        .unwrap_or("180".to_string())
        .parse()
        .unwrap_or(180);
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let _ = bulk_finalize(&mut transaction).await;
            let _ = commit_txn(transaction).await;
        }
        tokio::time::sleep(Duration::from_secs(finalize_sleep)).await;
    }
}
