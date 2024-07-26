use crate::database::daily_stat::bulk_finalize::bulk_finalize;
use crate::errors::error::Error;
use sqlx::PgPool;
use std::time::Duration;

pub async fn finalize_daily_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    loop {
        let mut transaction = pool.begin().await.map_err(Error::from)?;
        bulk_finalize(&mut transaction).await?;
        transaction.commit().await.map_err(Error::from)?;
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
