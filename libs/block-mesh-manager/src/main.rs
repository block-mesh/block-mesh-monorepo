#![forbid(unsafe_code)]
#![deny(elided_lifetimes_in_paths)]
#![deny(unreachable_pub)]

use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use block_mesh_manager::utils::cache_envar::get_envar;
    use database_utils::utils::migrate::migrate;
    use std::process;
    use dashmap::DashMap;
    use block_mesh_common::interfaces::server_api::{CheckTokenResponseMap, GetTokenResponseMap};
    use std::mem;
    use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
    use block_mesh_manager::database::user::create_test_user::create_test_user;
    use block_mesh_common::env::app_env_var::AppEnvVar;
    use block_mesh_common::env::env_var::EnvVar;
    use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
    use block_mesh_common::env::load_dotenv::load_dotenv;
    use std::env;
    #[allow(unused_imports)]
    use logger_general::tracing::setup_tracing_stdout_only;
    use std::time::Duration;
    use reqwest::ClientBuilder;
    use block_mesh_common::feature_flag_client::get_all_flags;
    #[cfg(not(target_env = "msvc"))]
    use tikv_jemallocator::Jemalloc;
    #[cfg(not(target_env = "msvc"))]
    #[global_allocator]
    static GLOBAL: Jemalloc = Jemalloc;
    use block_mesh_manager::configuration::get_configuration::get_configuration;
    use block_mesh_manager::emails::email_client::EmailClient;
    use block_mesh_manager::startup::application::{AppState, Application};
    use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
    use secret::Secret;
    use std::sync::Arc;
}}

#[cfg(feature = "ssr")]
#[tracing::instrument(name = "main", skip_all)]
fn main() {
    let sentry_layer = env::var("SENTRY_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let sentry_url = env::var("SENTRY").unwrap_or_default();
    let sentry_sample_rate = env::var("SENTRY_SAMPLE_RATE")
        .unwrap_or("0.1".to_string())
        .parse()
        .unwrap_or(0.1);
    if sentry_layer {
        let _guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                debug: env::var("APP_ENVIRONMENT").unwrap_or_default() == "local",
                sample_rate: sentry_sample_rate,
                traces_sample_rate: sentry_sample_rate,
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        mem::forget(_guard);
    }

    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await });
    process::exit(1);
}

#[cfg(feature = "ssr")]
#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    // setup_tracing_stdout_only();
    // console_subscriber::init(); // tokio-console
    setup_tracing_stdout_only_with_sentry();
    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let mailgun_token = get_env_var_or_panic(AppEnvVar::MailgunSendKey);
    let _mailgun_token = <EnvVar as AsRef<Secret<String>>>::as_ref(&mailgun_token);
    let db_pool = get_connection_pool(&configuration.database, Option::from(database_url)).await?;
    let env = get_envar("APP_ENVIRONMENT").await;
    tracing::info!("Database migration started");
    migrate(&db_pool, env)
        .await
        .expect("Failed to migrate database");
    tracing::info!("Database migration complete");
    let email_client = Arc::new(EmailClient::new(configuration.application.base_url.clone()).await);
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .cookie_store(true)
        .user_agent("curl/8.7.1")
        .build()
        .unwrap_or_default();
    tracing::info!("Starting to get feature flags");
    let flags = get_all_flags(&client).await?;
    tracing::info!("Finished getting feature flags");
    let redis_url = env::var("REDIS_URL")?;
    let redis_url = if redis_url.ends_with("#insecure") {
        redis_url
    } else {
        format!("{}#insecure", redis_url)
    };
    tracing::info!("Starting redis client");
    let redis_client = redis::Client::open(redis_url)?;
    tracing::info!("Found redis client URL");
    let redis = redis_client.get_multiplexed_async_connection().await?;
    tracing::info!("Finished redis client");

    let _ = create_test_user(&db_pool).await;

    let check_token_map: CheckTokenResponseMap = Arc::new(DashMap::new());
    let get_token_map: GetTokenResponseMap = Arc::new(DashMap::new());

    let app_state = Arc::new(AppState {
        check_token_map,
        get_token_map,
        email_client,
        pool: db_pool.clone(),
        client,
        flags,
        redis,
    });
    tracing::info!("Starting application server");
    let application = Application::build(configuration, app_state.clone(), db_pool.clone()).await;
    let application_task = tokio::spawn(application.run());

    tokio::select! {
        o = application_task => panic!("API {:?}", o),
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
