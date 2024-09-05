use crate::pg_listener::{start_listening, Payload};
use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY;
use block_mesh_common::env::load_dotenv::load_dotenv;
use logger_general::tracing::setup_tracing_stdout_only;

mod pg_listener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    // let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    // let redis_client = redis::Client::open(env::var("REDIS_URL")?)?;
    // let redis = redis_client.get_multiplexed_async_connection().await?;
    //
    // let call_back = |payload: Payload| {
    //     tracing::info!("Payload received: {:#?}", payload);
    // };
    // let db_listen = tokio::spawn(start_listening(
    //     db_pool.clone(),
    //     vec![BLOCKMESH_PG_NOTIFY],
    //     call_back,
    // ));
    Ok(())
}
