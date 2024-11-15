use crate::db_calls::bulk_task_bonus::bulk_task_bonus;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::time::Instant;

#[tracing::instrument(name = "bulk_task_bonus_cron", level = "trace", skip(pool))]
pub async fn bulk_task_bonus_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    let bonus = env::var("BULK_TASK_BONUS")
        .unwrap_or(String::from("0"))
        .parse()
        .unwrap_or(0i32);
    let limit = env::var("BULK_TASK_LIMIT")
        .unwrap_or(String::from("50"))
        .parse()
        .unwrap_or(50i32);
    let sleep = env::var("BULK_TASK_CRON_SLEEP")
        .unwrap_or(String::from("600000"))
        .parse()
        .unwrap_or(600000u64);
    let duration = Duration::from_millis(sleep);
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let now = Instant::now();
            let r = bulk_task_bonus(&mut transaction, bonus, limit).await;
            let _ = commit_txn(transaction).await;
            let elapsed = now.elapsed();
            if let Ok(r) = r {
                tracing::info!(
                    "bulk_task_bonus bonus = {}, limit = {} , affected rows = {}, elapsed = {:?}",
                    bonus,
                    limit,
                    r.rows_affected(),
                    elapsed
                );
            }
        }
        tokio::time::sleep(duration).await;
    }
}
