use block_mesh_common::feature_flag_client::get_all_flags;
use block_mesh_common::interfaces::server_api::{RegisterForm, RegisterResponse};
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager::configuration::get_configuration::get_configuration;
use block_mesh_manager::configuration::settings::Settings;
use block_mesh_manager::database::migrate::migrate;
use block_mesh_manager::emails::email_client::EmailClient;
use block_mesh_manager::envars::app_env_var::AppEnvVar;
use block_mesh_manager::envars::env_var::EnvVar;
use block_mesh_manager::envars::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_manager::envars::load_dotenv::load_dotenv;
use block_mesh_manager::startup::application::{AppState, Application};
use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
use block_mesh_manager::worker::db_agg::{db_agg, UpdateBulkMessage};
use block_mesh_manager::worker::db_cleaner_cron::{db_cleaner_cron, EnrichIp};
use block_mesh_manager::worker::finalize_daily_cron::finalize_daily_cron;
use block_mesh_manager::worker::joiner::joiner_loop;
use block_mesh_manager::worker::rpc_cron::rpc_worker_loop;
use logger_general::tracing::setup_tracing_stdout_only;
use redis;
use redis::aio::MultiplexedConnection;
use reqwest::ClientBuilder;
use secret::Secret;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio;
use tokio::task::JoinHandle;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub client: reqwest::Client,
    pub configuration: Settings,
    pub app_state: Arc<AppState>,
    pub redis: MultiplexedConnection,
}

pub async fn spawn_app() -> TestApp {
    setup_tracing_stdout_only();
    load_dotenv();

    let configuration = {
        let mut configuration = get_configuration().expect("Failed to read configuration");
        configuration.application.port = 0;
        configuration.database.name = Uuid::new_v4().to_string();
        configuration
    };
    tracing::info!("Starting with configuration {:#?}", configuration);
    let database_url = get_env_var_or_panic(AppEnvVar::DatabaseUrl);
    let database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);

    let db_pool = get_connection_pool(&configuration.database.clone(), Option::from(database_url))
        .await
        .unwrap();

    migrate(&db_pool).await.expect("Failed to migrate database");
    let email_client = Arc::new(EmailClient::new(configuration.application.base_url.clone()).await);
    let (tx, rx) = tokio::sync::mpsc::channel::<JoinHandle<()>>(500);
    let (tx_sql_agg, rx_sql_agg) = tokio::sync::mpsc::channel::<UpdateBulkMessage>(500);
    let (cleaner_tx, cleaner_rx) = tokio::sync::mpsc::unbounded_channel::<EnrichIp>();
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    let flags = get_all_flags(&client).await.unwrap();
    let redis_client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    let redis = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let app_state = Arc::new(AppState {
        email_client,
        pool: db_pool.clone(),
        client: client.clone(),
        tx,
        tx_sql_agg,
        flags,
        cleaner_tx,
        redis: redis.clone(),
    });
    let application =
        Application::build(configuration.clone(), app_state.clone(), db_pool.clone()).await;
    let address = format!("http://{}", application.address());
    let port = application.port();
    let _rpc_worker_task = tokio::spawn(rpc_worker_loop(db_pool.clone()));
    let _application_task = tokio::spawn(application.run());
    let _joiner_task = tokio::spawn(joiner_loop(rx));
    let _finalize_daily_stats_task = tokio::spawn(finalize_daily_cron(db_pool.clone()));
    let _db_cleaner_task = tokio::spawn(db_cleaner_cron(db_pool.clone(), cleaner_rx));
    let _db_agg_task = tokio::spawn(db_agg(db_pool.clone(), rx_sql_agg));

    TestApp {
        address,
        port,
        db_pool,
        client,
        configuration,
        app_state: app_state.clone(),
        redis,
    }
}

impl TestApp {
    pub async fn register_post(&self, form: &RegisterForm) -> RegisterResponse {
        let response = self
            .client
            .post(format!(
                "{}{}",
                &self.address,
                RoutesEnum::Static_UnAuth_Register
            ))
            .form(&form)
            .send()
            .await
            .expect("Failed register post HTTP");
        let response: RegisterResponse = response
            .json::<RegisterResponse>()
            .await
            .expect("Failed to deserialize RegisterResponse");
        response
    }
}
