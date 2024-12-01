use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = write_pool(None).await;
    let mut transaciton = create_txn(&pool).await?;
    health_check(&mut *transaciton).await?;
    commit_txn(transaciton).await?;
    println!("Hello, world!");
    Ok(())
}
