use crate::db_calls::bulk_uptime_bonus::bulk_uptime_bonus;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;

#[tracing::instrument(name = "bulk_uptime_bonus_cron", level = "trace", skip(pool))]
pub async fn bulk_uptime_bonus_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    let bonus = env::var("BULK_UPTIME_BONUS")
        .unwrap_or(String::from("0"))
        .parse()
        .unwrap_or(0f64);
    let sleep = env::var("BULK_UPTIME_CRON_SLEEP")
        .unwrap_or(String::from("60000"))
        .parse()
        .unwrap_or(60000u64);
    let duration = Duration::from_millis(sleep);
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let _ = bulk_uptime_bonus(&mut transaction, bonus).await;
            let _ = commit_txn(transaction).await;
        }
        tokio::time::sleep(duration).await;
    }
}
