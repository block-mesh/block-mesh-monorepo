use block_mesh_common::env::app_env_var::AppEnvVar;
use block_mesh_common::env::env_var::EnvVar;
use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_common::env::load_dotenv::load_dotenv;
use block_mesh_common::feature_flag_client::get_all_flags;
use block_mesh_common::interfaces::db_messages::{
    AggregateMessage, AnalyticsMessage, DailyStatMessage, UsersIpMessage,
};
use block_mesh_common::interfaces::server_api::{GetTokenRequest, GetTokenResponse, RegisterForm};
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager::configuration::get_configuration::get_configuration;
use block_mesh_manager::configuration::settings::Settings;
use block_mesh_manager::database::migrate::migrate;
use block_mesh_manager::emails::email_client::EmailClient;
use block_mesh_manager::startup::application::{AppState, Application};
use block_mesh_manager::startup::get_connection_pool::get_connection_pool;
use block_mesh_manager::worker::aggregate_agg::aggregate_agg;
use block_mesh_manager::worker::analytics_agg::analytics_agg;
use block_mesh_manager::worker::daily_stat_agg::daily_stat_agg;
use block_mesh_manager::worker::db_cleaner_cron::{db_cleaner_cron, EnrichIp};
use block_mesh_manager::worker::users_ip_agg::users_ip_agg;
use block_mesh_manager::ws::connection_manager::ConnectionManager;
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
use tokio::time::sleep;
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
    let (tx, rx) = flume::bounded::<JoinHandle<()>>(500);
    let (tx_daily_stat_agg, rx_daily_stat_agg) = flume::bounded::<DailyStatMessage>(500);
    let (tx_analytics_agg, rx_analytics_agg) = flume::bounded::<AnalyticsMessage>(500);
    let (tx_users_ip_agg, rx_users_ip_agg) = flume::bounded::<UsersIpMessage>(500);
    let (tx_aggregate_agg, rx_aggregate_agg) = flume::bounded::<AggregateMessage>(500);
    let (cleaner_tx, cleaner_rx) = flume::bounded::<EnrichIp>(500);

    let ws_connection_manager = ConnectionManager::new();
    let app_state = Arc::new(AppState {
        email_client,
        pool: db_pool.clone(),
        client,
        tx,
        tx_daily_stat_agg,
        tx_analytics_agg,
        flags,
        cleaner_tx,
        redis: redis.clone(),
        ws_connection_manager,
        tx_users_ip_agg,
        tx_aggregate_agg,
    });
    let application =
        Application::build(configuration.clone(), app_state.clone(), db_pool.clone()).await;
    let address = format!("http://{}", application.address());
    let port = application.port();

    let _db_cleaner_task = tokio::spawn(db_cleaner_cron(db_pool.clone(), cleaner_rx));
    let _db_daily_stat_task = tokio::spawn(daily_stat_agg(
        db_pool.clone(),
        rx_daily_stat_agg,
        app_state.clone(),
    ));
    let _db_analytics_task = tokio::spawn(analytics_agg(
        db_pool.clone(),
        rx_analytics_agg,
        app_state.clone(),
    ));
    let _db_users_ip_task = tokio::spawn(users_ip_agg(
        db_pool.clone(),
        rx_users_ip_agg,
        app_state.clone(),
    ));
    let _db_aggregate_task = tokio::spawn(aggregate_agg(
        db_pool.clone(),
        rx_aggregate_agg,
        app_state.clone(),
    ));

    sleep(Duration::from_secs(1)).await;
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

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
    pub fn ws_address(&self) -> String {
        let s = self.address.replace("http", "ws");
        format!("{}/ws", s)
    }

    pub fn ws_address_with_auth(&self, email: &str, api_token: &Uuid) -> String {
        let base = self.ws_address();
        format!("{base}?email={email}&api_token={api_token}")
    }

    pub async fn register_post(&self, form: &RegisterForm) -> anyhow::Result<()> {
        let response = self
            .client
            .post(format!(
                "{}{}",
                &self.address,
                RoutesEnum::Static_UnAuth_Register
            ))
            .form(&form)
            .send()
            .await?;
        assert_eq!(200, response.status());
        Ok(())
    }

    pub async fn get_api_token(&self, body: &GetTokenRequest) -> anyhow::Result<GetTokenResponse> {
        let response = self
            .client
            .post(format!("{}/api{}", &self.address, RoutesEnum::Api_GetToken))
            .json(&body)
            .send()
            .await?;
        assert_eq!(200, response.status());
        let response: GetTokenResponse = response.json::<GetTokenResponse>().await?;
        Ok(response)
    }
}
