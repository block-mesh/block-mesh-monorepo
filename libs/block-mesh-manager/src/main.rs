#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use block_mesh_common::interfaces::db_messages::UsersIpMessage;
    use block_mesh_manager::worker::users_ip_agg::users_ip_agg;
    use block_mesh_common::env::app_env_var::AppEnvVar;
    use block_mesh_common::env::env_var::EnvVar;
    use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
    use block_mesh_common::env::load_dotenv::load_dotenv;
    use block_mesh_manager::worker::aggregate_agg::{aggregate_agg, AggregateMessage};
    use block_mesh_manager::worker::daily_stat_agg::DailyStatMessage;
    // use block_mesh_manager::ws::connection_manager::ConnectionManager;
    use block_mesh_manager::worker::analytics_agg::{analytics_agg, AnalyticsMessage};
    use std::env;
    use block_mesh_manager::worker::daily_stat_agg::{daily_stat_agg};
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
    let (tx, rx) = flume::bounded::<JoinHandle<()>>(500);
    let (tx_daily_stat_agg, rx_daily_stat_agg) = flume::bounded::<DailyStatMessage>(500);
    let (tx_analytics_agg, rx_analytics_agg) = flume::bounded::<AnalyticsMessage>(500);
    let (tx_users_ip_agg, rx_users_ip_agg) = flume::bounded::<UsersIpMessage>(500);
    let (tx_aggregate_agg, rx_aggregate_agg) = flume::bounded::<AggregateMessage>(500);
    let (cleaner_tx, cleaner_rx) = flume::bounded::<EnrichIp>(500);
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    let flags = get_all_flags(&client).await?;
    let redis_client = redis::Client::open(env::var("REDIS_URL")?)?;
    let redis = redis_client.get_multiplexed_async_connection().await?;

    // let ws_connection_manager = ConnectionManager::new();
    let app_state = Arc::new(AppState {
        email_client,
        pool: db_pool.clone(),
        client,
        tx,
        tx_daily_stat_agg,
        tx_analytics_agg,
        flags,
        cleaner_tx,
        redis,
        // ws_connection_manager,
        tx_users_ip_agg,
        tx_aggregate_agg,
    });
    let application = Application::build(configuration, app_state, db_pool.clone()).await;
    let rpc_worker_task = tokio::spawn(rpc_worker_loop(db_pool.clone()));
    let application_task = tokio::spawn(application.run());
    let joiner_task = tokio::spawn(joiner_loop(rx));
    let finalize_daily_stats_task = tokio::spawn(finalize_daily_cron(db_pool.clone()));
    let db_cleaner_task = tokio::spawn(db_cleaner_cron(db_pool.clone(), cleaner_rx));
    let db_daily_stat_task = tokio::spawn(daily_stat_agg(db_pool.clone(), rx_daily_stat_agg));
    let db_analytics_task = tokio::spawn(analytics_agg(db_pool.clone(), rx_analytics_agg));
    let db_users_ip_task = tokio::spawn(users_ip_agg(db_pool.clone(), rx_users_ip_agg));
    let db_aggregate_task = tokio::spawn(aggregate_agg(db_pool.clone(), rx_aggregate_agg));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = rpc_worker_task =>  report_exit("RPC Background worker failed", o),
        o = joiner_task => report_exit("Joiner task failed", o),
        o = finalize_daily_stats_task => report_exit("Finalize daily task failed", o),
        o = db_cleaner_task => report_exit("DB cleaner task failed", o),
        o = db_daily_stat_task => report_exit("DB daily_stat aggregator", o),
        o = db_analytics_task => report_exit("DB analytics aggregator", o),
        o = db_users_ip_task => report_exit("DB users_ip aggregator", o),
        o = db_aggregate_task => report_exit("DB aggregate aggregator", o)
    };
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
