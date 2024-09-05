use crate::pg_listener::{start_listening, Payload};
use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY;
use block_mesh_common::env::load_dotenv::load_dotenv;
use logger_general::tracing::setup_tracing_stdout_only;
use std::env;

mod pg_listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    tracing::info!("Starting worker");
    let db_pool = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let redis_client = redis::Client::open(env::var("REDIS_URL")?)?;
    let _redis = redis_client.get_multiplexed_async_connection().await?;

    let call_back = |payload: Payload| {
        tracing::info!("Payload received: {:#?}", payload);
    };
    start_listening(db_pool.clone(), vec![BLOCKMESH_PG_NOTIFY], call_back).await?;
    Ok(())
}
