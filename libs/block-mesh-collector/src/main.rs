use aws_sdk_s3::{Client, config::Credentials};

mod collector_data;
mod cron_jobs;
mod errors;
mod routes;

use crate::cron_jobs::{get_products, upload_to_r2};
use crate::routes::get_router;
use aws_sdk_s3::config::{Region, SharedCredentialsProvider};
use axum::Router;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::migrate::migrate;
use logger_general::tracing::{get_sentry_layer, setup_tracing_stdout_only_with_sentry};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, mem, process};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

fn main() {
    let sentry_layer = get_sentry_layer();
    let sentry_url = env::var("SENTRY_DATA_SINK").unwrap_or_default();
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
    tracing::error!("block mesh manager worker stopped, exiting with exit code 1");
    process::exit(1);
}

#[derive(Clone)]
pub struct CollectorAppState {
    pub db_pool: PgPool,
    pub environment: Environment,
    pub r2_client: Client,
}

impl CollectorAppState {
    pub async fn new() -> Self {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let db_pool = write_pool(None).await;
        let r2_access_key = env::var("R2_ACCESS_KEY").unwrap();
        let r2_secret_key = env::var("R2_SECRET_KEY").unwrap();
        let r2_account_id = env::var("R2_ACCOUNT_ID").unwrap();
        let endpoint = format!("https://{}.r2.cloudflarestorage.com", r2_account_id);
        let credentials = Credentials::new(r2_access_key, r2_secret_key, None, None, "R2");
        let cred_provider = SharedCredentialsProvider::new(credentials);
        let region = Region::new("auto");
        let config = aws_config::SdkConfig::builder()
            .credentials_provider(cred_provider)
            .region(region)
            .endpoint_url(&endpoint)
            .build();
        let r2_client = Client::new(&config);
        Self {
            environment,
            db_pool,
            r2_client,
        }
    }
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    let state = CollectorAppState::new().await;
    let db_pool = Arc::new(state.db_pool.clone());
    let client = Arc::new(state.r2_client.clone());
    let env = env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT is not set");
    migrate(&state.db_pool, env)
        .await
        .expect("Failed to migrate database");
    let router = get_router(state);
    let cors = CorsLayer::permissive();
    let app = Router::new().nest("/", router).layer(cors);
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    let products_task = tokio::spawn(get_products(db_pool.clone()));
    let r2_task = tokio::spawn(upload_to_r2(db_pool, client));
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);
    tokio::select! {
        o = server_task => panic!("server task exit {:?}", o),
        o = products_task => panic!("products task exit {:?}", o),
        o = r2_task => panic!("r2_task task exit {:?}", o)
    }
}
