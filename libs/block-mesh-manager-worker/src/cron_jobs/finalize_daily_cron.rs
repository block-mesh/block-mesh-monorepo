use crate::db_calls::bulk_finalize::bulk_finalize;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::time::Duration;

#[tracing::instrument(name = "finalize_daily_cron", level = "trace", skip(pool))]
pub async fn finalize_daily_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let _ = bulk_finalize(&mut transaction).await;
            let _ = commit_txn(transaction).await;
        }
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
