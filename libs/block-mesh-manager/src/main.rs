#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

use block_mesh_manager::ws::connection_manager::ConnectionManager;
use cfg_if::cfg_if;

#[macro_use]
extern crate tracing;

cfg_if! { if #[cfg(feature = "ssr")] {
    use block_mesh_common::interfaces::ws_api::WsMessage;
    use block_mesh_manager::worker::ws_worker::{ws_worker_rx, ws_worker_tx};
    use tokio::sync::broadcast;
    use block_mesh_manager::worker::analytics_agg::{analytics_agg, AnalyticsMessage};
    use std::env;
    use block_mesh_manager::worker::db_agg::{db_agg, UpdateBulkMessage};
    use logger_general::tracing::setup_tracing_stdout_only;
    use std::time::Duration;
    use reqwest::ClientBuilder;
    use block_mesh_manager::worker::db_cleaner_cron::{db_cleaner_cron, EnrichIp};
    use block_mesh_manager::worker::finalize_daily_cron::finalize_daily_cron;
    use block_mesh_common::feature_flag_client::get_all_flags;
    use tokio::task::JoinHandle;
    use block_mesh_manager::worker::joiner::joiner_loop;
    #[cfg(not(target_env = "msvc"))]
    use tikv_jemallocator::Jemalloc;
    #[cfg(not(target_env = "msvc"))]
    #[global_allocator]
    static GLOBAL: Jemalloc = Jemalloc;
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
}}

#[cfg(feature = "ssr")]
fn main() {
    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await });
}

#[cfg(feature = "ssr")]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only();
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let mailgun_token = get_env_var_or_panic(AppEnvVar::MailgunSendKey);
    let _mailgun_token = <EnvVar as AsRef<Secret<String>>>::as_ref(&mailgun_token);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    migrate(&db_pool).await.expect("Failed to migrate database");
    let email_client = Arc::new(EmailClient::new(configuration.application.base_url.clone()).await);
    let (tx, rx) = tokio::sync::mpsc::channel::<JoinHandle<()>>(500);
    let (tx_ws, rx_ws) = broadcast::channel::<WsMessage>(500);
    let (tx_sql_agg, rx_sql_agg) = tokio::sync::mpsc::channel::<UpdateBulkMessage>(500);
    let (tx_analytics_agg, rx_analytics_agg) = tokio::sync::mpsc::channel::<AnalyticsMessage>(500);
    let (cleaner_tx, cleaner_rx) = tokio::sync::mpsc::unbounded_channel::<EnrichIp>();
    let (cleaner_tx, cleaner_rx) = tokio::sync::mpsc::channel::<EnrichIp>(500);
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    let flags = get_all_flags(&client).await?;
    let redis_client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    let redis = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let ws_connection_manager = ConnectionManager::new();
    let app_state = Arc::new(AppState {
        email_client,
        pool: db_pool.clone(),
        client,
        tx,
        tx_ws: tx_ws.clone(),
        rx_ws: rx_ws.resubscribe(),
        tx_sql_agg,
        tx_analytics_agg,
        flags,
        cleaner_tx,
        redis,
        ws_connection_manager,
    });
    let application = Application::build(configuration, app_state, db_pool.clone()).await;
    let rpc_worker_task = tokio::spawn(rpc_worker_loop(db_pool.clone()));
    let application_task = tokio::spawn(application.run());
    let joiner_task = tokio::spawn(joiner_loop(rx));
    let finalize_daily_stats_task = tokio::spawn(finalize_daily_cron(db_pool.clone()));
    let db_cleaner_task = tokio::spawn(db_cleaner_cron(db_pool.clone(), cleaner_rx));
    let db_agg_task = tokio::spawn(db_agg(db_pool.clone(), rx_sql_agg));
    let ws_task_rx = tokio::spawn(ws_worker_rx(
        db_pool.clone(),
        rx_ws.resubscribe(),
        tx_ws.clone(),
    ));
    let ws_task_tx = tokio::spawn(ws_worker_tx(db_pool.clone(), rx_ws, tx_ws));
    let db_analytics_task = tokio::spawn(analytics_agg(db_pool.clone(), rx_analytics_agg));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = rpc_worker_task =>  report_exit("RPC Background worker failed", o),
        o = joiner_task => report_exit("Joiner task failed", o),
        o = finalize_daily_stats_task => report_exit("Finalize daily task failed", o),
        o = db_cleaner_task => report_exit("DB cleaner task failed", o),
        o = db_agg_task => report_exit("DB aggregator", o),
        o = db_analytics_task => report_exit("DB analytics aggregator", o),
        o = ws_task_rx => report_exit("WS RX task failed", o),
        o = ws_task_tx => report_exit("WS TX task failed", o)
    };
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
