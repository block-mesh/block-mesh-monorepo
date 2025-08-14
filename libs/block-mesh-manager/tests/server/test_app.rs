use block_mesh_common::constants::DeviceType;
use block_mesh_common::email_client::client::EmailClient;
use block_mesh_common::env::app_env_var::AppEnvVar;
use block_mesh_common::env::env_var::EnvVar;
use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_common::env::load_dotenv::load_dotenv;
use block_mesh_common::feature_flag_client::get_all_flags;
use block_mesh_common::interfaces::server_api::{
    CheckTokenResponseMap, GetTokenRequest, GetTokenResponse, GetTokenResponseMap, RegisterForm,
};
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager::configuration::get_configuration::get_configuration;
use block_mesh_manager::configuration::settings::Settings;
use block_mesh_manager::startup::application::{AppState, Application};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use dash_with_expiry::hash_set_with_expiry::HashSetWithExpiry;
use dashmap::DashMap;
use database_utils::utils::connection::channel_pool::channel_pool;
use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::migrate::migrate;
use logger_general::tracing::setup_tracing_stdout_only;
use redis;
use redis::aio::MultiplexedConnection;
use reqwest::ClientBuilder;
use secret::Secret;
use sqlx::PgPool;
use std::env;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio;
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
    let _database_url = <EnvVar as AsRef<Secret<String>>>::as_ref(&database_url);
    let db_pool = write_pool(None).await;
    let channel_pool = channel_pool(Some("CHANNEL_DATABASE_URL".to_string())).await;

    migrate(&db_pool, "test".to_string())
        .await
        .expect("Failed to migrate database");
    let email_client = Arc::new(EmailClient::new(configuration.application.base_url.clone()).await);
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();

    let flags = Arc::new(RwLock::new(
        get_all_flags(&client, DeviceType::AppServer).await.unwrap(),
    ));
    let redis_client = redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap();
    let redis = redis_client
        .get_multiplexed_async_connection()
        .await
        .unwrap();

    let check_token_map: CheckTokenResponseMap = Arc::new(DashMap::new());
    let get_token_map: GetTokenResponseMap = Arc::new(DashMap::new());
    let wallet_addresses = HashMapWithExpiry::new(1_000);
    let invite_codes = HashMapWithExpiry::new(1_000);

    let app_state = Arc::new(AppState {
        wallet_login_nonce: HashMapWithExpiry::new(1_000),
        rate_limiter: HashSetWithExpiry::new(),
        enable_hcaptcha: false,
        enable_recaptcha: false,
        enable_proof_of_humanity: false,
        hcaptcha_site_key: "h".to_string(),
        hcaptcha_secret_key: "h".to_string(),
        cf_enforce: false,
        recaptcha_secret_key_v2: "v2".to_string(),
        recaptcha_secret_key_v3: "v3".to_string(),
        recaptcha_site_key_v2: "v2".to_string(),
        recaptcha_site_key_v3: "v3".to_string(),
        cf_site_key: "1".to_string(),
        cf_secret_key: "2".to_string(),
        invite_codes,
        wallet_addresses,
        rate_limit: true,
        task_limit: true,
        submit_bandwidth_limit: true,
        get_token_map,
        email_client,
        pool: db_pool.clone(),
        follower_pool: db_pool.clone(),
        channel_pool,
        client,
        flags,
        redis: redis.clone(),
        check_token_map,
    });
    let application =
        Application::build(configuration.clone(), app_state.clone(), db_pool.clone()).await;
    let address = format!("http://{}", application.address());
    let port = application.port();

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
