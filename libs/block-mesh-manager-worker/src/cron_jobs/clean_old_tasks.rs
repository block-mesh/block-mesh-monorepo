use crate::db_calls::bulk_delete_old_tasks::bulk_delete_old_tasks;
use sqlx::PgPool;
use std::time::Duration;

#[tracing::instrument(name = "clean_old_tasks", level = "trace", skip(pool))]
pub async fn clean_old_tasks(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        if let Ok(mut transaction) = pool.begin().await {
            let _ = bulk_delete_old_tasks(&mut transaction).await;
            let _ = transaction.commit().await;
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
