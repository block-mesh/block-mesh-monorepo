use anyhow::anyhow;
use sqlx::{PgPool, Postgres, Transaction};
use std::env;
use tokio::sync::OnceCell;
use tokio::time::timeout;

static DURATION: OnceCell<u64> = OnceCell::const_new();

#[tracing::instrument(name = "get_timeout_duration", skip_all)]
async fn get_timeout_duration() -> tokio::time::Duration {
    let duration = DURATION
        .get_or_init(|| async {
            env::var("INSTRUMENT_WRAPPER")
                .unwrap_or("1000".to_string())
                .parse()
                .unwrap_or(1000)
        })
        .await;
    tokio::time::Duration::from_millis(*duration)
}

#[tracing::instrument(name = "create_txn", skip_all, err)]
pub async fn create_txn(pool: &PgPool) -> anyhow::Result<Transaction<'_, Postgres>> {
    let duration = get_timeout_duration().await;
    match timeout(duration, pool.begin()).await {
        Ok(txn) => match txn {
            Ok(transaction) => Ok(transaction),
            Err(e) => Err(anyhow!("TXN => create_txn error {}", e.to_string())),
        },
        Err(e) => Err(anyhow!("OTHER create_txn error {}", e.to_string())),
    }
}

#[tracing::instrument(name = "create_txn_with_timeout", skip_all, err)]
pub async fn create_txn_with_timeout(
    pool: &PgPool,
    timeout_period: u64,
) -> anyhow::Result<Transaction<'_, Postgres>> {
    let duration = tokio::time::Duration::from_millis(timeout_period);
    match timeout(duration, pool.begin()).await {
        Ok(txn) => match txn {
            Ok(transaction) => Ok(transaction),
            Err(e) => Err(anyhow!("TXN => create_txn error {}", e.to_string())),
        },
        Err(e) => Err(anyhow!("OTHER create_txn error {}", e.to_string())),
    }
}

#[tracing::instrument(name = "commit_txn", skip_all, err)]
pub async fn commit_txn(txn: Transaction<'_, Postgres>) -> anyhow::Result<()> {
    let duration = get_timeout_duration().await;
    match timeout(duration, txn.commit()).await {
        Ok(txn) => match txn {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("TXN => commit_txn error {}", e.to_string())),
        },
        Err(e) => Err(anyhow!("OTHER commit_txn error {}", e.to_string())),
    }
}

#[tracing::instrument(name = "commit_txn_with_timeout", skip_all, err)]
pub async fn commit_txn_with_timeout(
    txn: Transaction<'_, Postgres>,
    timeout_period: u64,
) -> anyhow::Result<()> {
    let duration = tokio::time::Duration::from_millis(timeout_period);
    match timeout(duration, txn.commit()).await {
        Ok(txn) => match txn {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("TXN => commit_txn error {}", e.to_string())),
        },
        Err(e) => Err(anyhow!("OTHER commit_txn error {}", e.to_string())),
    }
}
