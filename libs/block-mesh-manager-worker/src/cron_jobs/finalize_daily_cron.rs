use crate::db_calls::bulk_finalize::bulk_finalize;
use anyhow::Context;
use sqlx::PgPool;
use std::time::Duration;

pub async fn finalize_daily_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        let mut transaction = pool.begin().await.context("Cannot create DB transaction")?;
        bulk_finalize(&mut transaction).await?;
        transaction
            .commit()
            .await
            .context("Cannot commit DB transaction")?;
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
