#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use tokio::task::JoinHandle;
    use block_mesh_manager::worker::joiner::joiner_loop;
    #[cfg(not(target_env = "msvc"))]
    use tikv_jemallocator::Jemalloc;
    #[cfg(not(target_env = "msvc"))]
    #[global_allocator]
    static GLOBAL: Jemalloc = Jemalloc;
    use logger_general::tracing::setup_tracing;
    use block_mesh_common::constants::{DeviceType, BLOCKMESH_SERVER_UUID_ENVAR};
    use block_mesh_manager::configuration::get_configuration::get_configuration;
    use block_mesh_manager::database::migrate::migrate;
    use block_mesh_manager::emails::email_client::EmailClient;
    use block_mesh_manager::envars::app_env_var::AppEnvVar;
    use block_mesh_manager::envars::env_var::EnvVar;
    use block_mesh_manager::envars::get_env_var_or_panic::get_env_var_or_panic;
    use block_mesh_manager::envars::load_dotenv::load_dotenv;
    use block_mesh_manager::startup::application::{AppState, Application};
    use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
    use block_mesh_manager::startup::report_exit::report_exit;
    use block_mesh_manager::worker::rpc_cron::rpc_worker_loop;
    use secret::Secret;
    use std::sync::Arc;
    use uuid::Uuid;
    use reqwest::Client;
}}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing(
        Uuid::parse_str(std::env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap(),
        DeviceType::AppServer,
    );
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let mailgun_token = get_env_var_or_panic(AppEnvVar::MailgunSendKey);
    let mailgun_token = <EnvVar as AsRef<Secret<String>>>::as_ref(&mailgun_token);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    migrate(&db_pool).await.expect("Failed to migrate database");
    let email_client = Arc::new(EmailClient::new(
        mailgun_token.clone(),
        configuration.application.base_url.clone(),
    ));
    let (tx, rx) = tokio::sync::mpsc::channel::<JoinHandle<()>>(10);

    let app_state = Arc::new(AppState {
        email_client,
        pool: db_pool.clone(),
        client: Client::new(),
        tx,
    });
    let application = Application::build(configuration, app_state, db_pool.clone()).await;
    let rpc_worker_task = tokio::spawn(rpc_worker_loop(db_pool.clone()));
    let application_task = tokio::spawn(application.run());
    let joiner_task = tokio::spawn(joiner_loop(rx));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = rpc_worker_task =>  report_exit("RPC Background worker failed", o),
        o = joiner_task => report_exit("Joiner task failed", o),
    };
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
