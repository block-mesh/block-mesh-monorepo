use crate::db_calls::bulk_delete_old_tasks::bulk_delete_old_tasks;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::time::Duration;

#[tracing::instrument(name = "clean_old_tasks", level = "trace", skip(pool))]
pub async fn clean_old_tasks(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            let _ = bulk_delete_old_tasks(&mut transaction).await;
            let _ = commit_txn(transaction).await;
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
