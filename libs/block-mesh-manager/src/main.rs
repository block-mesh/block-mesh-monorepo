#![forbid(unsafe_code)]

use block_mesh_manager::configuration::get_configuration::get_configuration;
use block_mesh_manager::domain::secret::Secret;
use block_mesh_manager::envars::app_env_var::AppEnvVar;
use block_mesh_manager::envars::env_var::EnvVar;
use block_mesh_manager::envars::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_manager::envars::load_dotenv::load_dotenv;
use block_mesh_manager::startup::application::{AppState, Application};
use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
use block_mesh_manager::startup::report_exit::report_exit;
use block_mesh_manager::telemetry::subscriber::{get_subscriber, init_subscriber};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    let configuration = get_configuration().expect("Failed to read configuration");
    let subscriber = get_subscriber(
        "block-mesh-manager",
        if <EnvVar as AsRef<String>>::as_ref(&get_env_var_or_panic(AppEnvVar::AppEnvironment))
            == "local"
        {
            "info"
        } else {
            "debug"
        },
        std::io::stdout,
        false,
    );
    init_subscriber(subscriber);
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
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
