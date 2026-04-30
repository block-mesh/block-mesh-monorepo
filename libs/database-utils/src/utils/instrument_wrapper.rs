use anyhow::anyhow;
use sqlx::{PgPool, Postgres, Transaction};

#[tracing::instrument(name = "create_txn", skip_all, err)]
pub async fn create_txn(pool: &PgPool) -> anyhow::Result<Transaction<'_, Postgres>> {
    match pool.begin().await {
        Ok(transaction) => Ok(transaction),
        Err(e) => Err(anyhow!("TXN => create_txn error {e}")),
    }
}

#[tracing::instrument(name = "create_txn_with_timeout", skip_all, err)]
pub async fn create_txn_with_timeout(
    pool: &PgPool,
    _timeout_period: u64,
) -> anyhow::Result<Transaction<'_, Postgres>> {
    create_txn(pool).await
}

#[tracing::instrument(name = "commit_txn", skip_all, err)]
pub async fn commit_txn(txn: Transaction<'_, Postgres>) -> anyhow::Result<()> {
    match txn.commit().await {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("TXN => commit_txn error {e}")),
    }
}

#[tracing::instrument(name = "commit_txn_with_timeout", skip_all, err)]
pub async fn commit_txn_with_timeout(
    txn: Transaction<'_, Postgres>,
    _timeout_period: u64,
) -> anyhow::Result<()> {
    commit_txn(txn).await
}
