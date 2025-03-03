use crate::db_calls::bulk_uptime_bonus::bulk_uptime_bonus;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::time::Instant;

#[tracing::instrument(name = "bulk_uptime_bonus_cron", level = "trace", skip(pool))]
pub async fn bulk_uptime_bonus_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    let enable = env::var("BULK_UPTIME_BONUS_ENABLE")
        .unwrap_or("false".to_ascii_lowercase())
        .parse()
        .unwrap_or(false);
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
        if enable {
            if let Ok(mut transaction) = create_txn(&pool).await {
                let now = Instant::now();

                let r = bulk_uptime_bonus(&mut transaction, bonus).await;
                let _ = commit_txn(transaction).await;
                let elapsed = now.elapsed();
                if let Ok(r) = r {
                    tracing::info!(
                        "bulk_uptime_bonus bonus = {} , affected rows = {}, elapsed = {:?}",
                        bonus,
                        r.rows_affected(),
                        elapsed
                    );
                }
            }
        }
        tokio::time::sleep(duration).await;
    }
}
