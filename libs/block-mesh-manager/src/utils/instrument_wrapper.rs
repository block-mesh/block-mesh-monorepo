use crate::errors::error::Error;
use sqlx::{PgPool, Postgres, Transaction};

#[tracing::instrument(name = "create_txn", skip_all)]
pub async fn create_txn(pool: &PgPool) -> anyhow::Result<Transaction<'_, Postgres>> {
    let transaction = pool.begin().await.map_err(Error::from)?;
    Ok(transaction)
}

#[tracing::instrument(name = "commit_txn", skip_all)]
pub async fn commit_txn(txn: Transaction<'_, Postgres>) -> anyhow::Result<()> {
    Ok(txn.commit().await.map_err(Error::from)?)
}
