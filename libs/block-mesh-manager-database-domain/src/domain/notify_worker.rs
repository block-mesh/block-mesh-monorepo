use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY_WORKER;
use block_mesh_common::interfaces::db_messages::DBMessage;
use sqlx::PgPool;

#[tracing::instrument(name = "notify_worker", skip_all)]
pub async fn notify_worker(pool: &PgPool, messages: &Vec<DBMessage>) -> anyhow::Result<()> {
    let s = serde_json::to_string(&messages)?.replace('\'', "\"");
    let q = format!("NOTIFY {BLOCKMESH_PG_NOTIFY_WORKER} , '{s}'");
    sqlx::query(&q).execute(pool).await?;
    Ok(())
}
