use crate::db_calls::bulk_task_bonus::bulk_task_bonus;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;

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
            let _ = bulk_task_bonus(&mut transaction, bonus, limit).await;
            let _ = commit_txn(transaction).await;
        }
        tokio::time::sleep(duration).await;
    }
}
