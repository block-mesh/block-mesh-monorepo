use crate::db_calls::create_task::create_task;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

pub async fn create_special_task_cron(pool: &PgPool) -> anyhow::Result<()> {
    let limit = env::var("SPECIAL_CRON_LIMIT")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let url = env::var("SPECIAL_URL")?;
    let uuid = Uuid::parse_str(env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();

    let mut transaction = create_txn(pool).await?;
    for _ in 0..limit {
        let _ = create_task(&mut transaction, &uuid, &url, "GET", None, None).await?;
    }
    _ = commit_txn(transaction).await?;
    Ok(())
}

pub async fn special_worker_loop(pool: PgPool) -> Result<(), anyhow::Error> {
    let interval = env::var("SPECIAL_CRON_INTERVAL")
        .unwrap_or("30000".to_string())
        .parse()
        .unwrap_or(30_000);
    loop {
        let _ = create_special_task_cron(&pool).await;
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}
