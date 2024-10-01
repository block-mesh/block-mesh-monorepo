use crate::db_calls::create_server_user::create_server_user;
use crate::db_calls::get_all_rpcs::get_all_rpcs;
use anyhow::anyhow;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "create_rpc_tasks", level = "trace", skip(pool))]
pub async fn create_rpc_tasks(pool: PgPool) -> anyhow::Result<()> {
    let mut transaction = create_txn(&pool).await?;
    let uuid = Uuid::parse_str(env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    for rpc in get_all_rpcs(&mut transaction).await? {
        rpc.create_rpc_task(&mut transaction, &uuid).await?;
    }
    commit_txn(transaction).await
}

#[tracing::instrument(name = "rpc_worker_loop", skip(pool))]
pub async fn rpc_worker_loop(pool: PgPool) -> Result<(), anyhow::Error> {
    let interval = env::var("RPC_CRON_INTERVAL")
        .unwrap_or("30000".to_string())
        .parse()
        .unwrap_or(30_000);
    if let Ok(mut transaction) = create_txn(&pool).await {
        _ = create_server_user(&mut transaction).await;
        _ = commit_txn(transaction).await;
    }
    loop {
        match create_rpc_tasks(pool.clone()).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("worker_loop: create_rpc_tasks: error: {}", e);
            }
        }
        tokio::time::sleep(Duration::from_millis(interval)).await;
    }
}
