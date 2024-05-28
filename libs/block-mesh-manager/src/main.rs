#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

use block_mesh_common::tracing::setup_tracing;
use block_mesh_manager::configuration::get_configuration::get_configuration;
use block_mesh_manager::database::migrate::migrate;
use block_mesh_manager::envars::app_env_var::AppEnvVar;
use block_mesh_manager::envars::env_var::EnvVar;
use block_mesh_manager::envars::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_manager::envars::load_dotenv::load_dotenv;
use block_mesh_manager::startup::application::{AppState, Application};
use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
use block_mesh_manager::startup::report_exit::report_exit;
use secret::Secret;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing(
        Uuid::parse_str(
            std::env::var("BLOCKMESH_SERVER_UUID_ENVAR")
                .unwrap()
                .as_str(),
        )
        .unwrap(),
    );
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    migrate(&db_pool).await.expect("Failed to migrate database");
    let app_state = Arc::new(AppState {
        pool: db_pool.clone(),
    });
    let application = Application::build(configuration, app_state, db_pool).await;
    let application_task = tokio::spawn(application.run());
    tokio::select! {
        o = application_task => report_exit("API", o),
        // o = worker_task =>  report_exit("Background worker", o),
    };
    Ok(())
}
