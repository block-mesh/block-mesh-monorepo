mod aggregator;
mod data_sink;
mod database;
mod errors;
mod migrate_clickhouse;
mod routes;

use crate::aggregator::{collect_writes_for_clickhouse, joiner_loop};
use crate::data_sink::DataSinkClickHouse;
use crate::migrate_clickhouse::migrate_clickhouse;
use crate::routes::get_router;
use axum::Router;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::env::load_dotenv::load_dotenv;
use clickhouse::Client;
use database_utils::utils::connection::follower_pool::follower_pool;
// use database_utils::utils::connection::write_pool::write_pool;
// use database_utils::utils::migrate::migrate;
use anyhow::anyhow;
use flume::Sender;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use solana_sdk::signature::Keypair;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, mem, process};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;

pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

fn main() {
    let sentry_layer = env::var("SENTRY_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
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
pub struct DataSinkAppState {
    pub clickhouse_client: Arc<Client>,
    pub follower_db_pool: PgPool,
    pub environment: Environment,
    pub use_clickhouse: bool,
    pub tx: Sender<DataSinkClickHouse>,
    pub ext_keypair: Arc<Keypair>,
    pub enforce_signature: bool,
}

const EXT_KEYPAIR: &'static str = env!("EXT_KEYPAIR");

pub fn get_keypair() -> anyhow::Result<Keypair> {
    let data: serde_json::Value =
        serde_json::from_str(EXT_KEYPAIR).map_err(|e| anyhow!(e.to_string()))?;
    let key_bytes: Vec<u8> = serde_json::from_value(data).map_err(|e| anyhow!(e.to_string()))?;
    Ok(Keypair::from_bytes(&key_bytes).map_err(|e| anyhow!(e.to_string()))?)
}

impl DataSinkAppState {
    pub async fn new(tx: Sender<DataSinkClickHouse>) -> Self {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let enforce_signature = env::var("ENFORCE_SIGNATURE")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let ext_keypair = get_keypair().unwrap();
        let use_clickhouse = env::var("USE_CLICKHOUSE")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let environment = Environment::from_str(&environment).unwrap();
        // https://clickhouse.com/docs/en/operations/settings/settings#async-insert
        // https://clickhouse.com/docs/en/operations/settings/settings#wait-for-async-insert
        let clickhouse_client = if environment == Environment::Production {
            Arc::new(
                Client::default()
                    .with_url(env::var("CLICKHOUSE_URL").unwrap())
                    .with_user(env::var("CLICKHOUSE_USER").unwrap())
                    .with_password(env::var("CLICKHOUSE_PASSWORD").unwrap())
                    .with_option("async_insert", "1")
                    .with_option("wait_for_async_insert", "0"),
            )
        } else {
            Arc::new(
                Client::default()
                    .with_url(env::var("CLICKHOUSE_URL").unwrap())
                    .with_option("async_insert", "1")
                    .with_option("wait_for_async_insert", "0"),
            )
        };
        let follower_db_pool = follower_pool(Some("FOLLOWER_DATABASE_URL".to_string())).await;
        Self {
            ext_keypair: Arc::new(ext_keypair),
            tx,
            use_clickhouse,
            clickhouse_client,
            follower_db_pool,
            environment,
            enforce_signature,
        }
    }
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    tracing::info!("Starting worker");
    let (tx, rx) = flume::bounded::<DataSinkClickHouse>(
        env::var("BROADCAST_CHANNEL_SIZE")
            .unwrap_or("5000".to_string())
            .parse()
            .unwrap_or(5000),
    );
    let state = DataSinkAppState::new(tx).await;
    let clickhouse_client = state.clickhouse_client.clone();
    let _env = env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT is not set");
    let (joiner_tx, joiner_rx) = flume::bounded::<JoinHandle<()>>(500);
    // migrate(&state.data_sink_db_pool, env)
    //     .await
    //     .expect("Failed to migrate database");
    migrate_clickhouse(&state.clickhouse_client)
        .await
        .expect("Failed to migrate clickhouse");

    let collect_writes_for_clickhouse_task = tokio::spawn(collect_writes_for_clickhouse(
        clickhouse_client,
        joiner_tx,
        rx,
        env::var("AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));

    let router = get_router(state);
    let cors = CorsLayer::permissive();
    let app = Router::new().nest("/", router).layer(cors);
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);
    let joiner_task = tokio::spawn(joiner_loop(joiner_rx));
    tokio::select! {
        o = collect_writes_for_clickhouse_task => panic!("collect_writes_for_clickhouse_task exit {:?}", o),
        o = joiner_task => panic!("joiner_task exit {:?}", o),
        o = server_task => panic!("server task exit {:?}", o)
    }
}
