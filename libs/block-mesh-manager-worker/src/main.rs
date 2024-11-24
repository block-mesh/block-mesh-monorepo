use crate::db_aggregators::users_ip_aggregator::users_ip_aggregator;
use crate::pg_listener::start_listening;
use axum::{Extension, Router};
use block_mesh_common::constants::BLOCKMESH_PG_NOTIFY_WORKER;
use block_mesh_common::env::load_dotenv::load_dotenv;
use database_utils::utils::connection::channel_pool::channel_pool;
use database_utils::utils::connection::unlimited_pool::unlimited_pool;
use database_utils::utils::connection::write_pool::write_pool;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use serde_json::Value;
use std::net::SocketAddr;
use std::{env, mem, process};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;

mod call_backs;
mod cron_jobs;
mod db_aggregators;
mod db_calls;
mod domain;
mod errors;
mod pg_listener;
mod routes;
mod utils;

use crate::call_backs::send_to_rx::send_to_rx;
use crate::cron_jobs::bulk_task_bonus_cron::bulk_task_bonus_cron;
use crate::cron_jobs::bulk_uptime_bonus_cron::bulk_uptime_bonus_cron;
use crate::cron_jobs::clean_old_tasks::clean_old_tasks;
use crate::cron_jobs::finalize_daily_cron::finalize_daily_cron;
use crate::cron_jobs::rpc_cron::rpc_worker_loop;
use crate::cron_jobs::special_task_cron::special_worker_loop;
use crate::db_aggregators::add_to_aggregates_aggregator::add_to_aggregates_aggregator;
use crate::db_aggregators::aggregates_aggregator::aggregates_aggregator;
use crate::db_aggregators::analytics_aggregator::analytics_aggregator;
use crate::db_aggregators::daily_stats_aggregator::daily_stats_aggregator;
use crate::db_aggregators::joiner_loop::joiner_loop;
use crate::routes::get_router;

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
    let sentry_url = env::var("SENTRY_WORKER").unwrap_or_default();
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

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    tracing::info!("Starting worker");
    let db_pool = write_pool(None).await;
    let un_limited_db_pool = unlimited_pool(None).await;
    // let redis_client = redis::Client::open(env::var("REDIS_URL")?)?;
    // let _redis = redis_client.get_multiplexed_async_connection().await?;
    let (joiner_tx, joiner_rx) = flume::bounded::<JoinHandle<()>>(500);
    let (tx, _rx) = tokio::sync::broadcast::channel::<Value>(
        env::var("BROADCAST_CHANNEL_SIZE")
            .unwrap_or("5000".to_string())
            .parse()
            .unwrap_or(5000),
    );

    let bulk_task_bonus_task = tokio::spawn(bulk_task_bonus_cron(un_limited_db_pool.clone()));
    let bulk_uptime_bonus_task = tokio::spawn(bulk_uptime_bonus_cron(un_limited_db_pool));
    let joiner_task = tokio::spawn(joiner_loop(joiner_rx));
    let rpc_worker_task = tokio::spawn(rpc_worker_loop(db_pool.clone()));
    let finalize_daily_stats_task = tokio::spawn(finalize_daily_cron(db_pool.clone()));
    let delete_old_tasks_task = tokio::spawn(clean_old_tasks(db_pool.clone()));
    let channel_pool = channel_pool(Some("CHANNEL_DATABASE_URL".to_string())).await;

    let db_listen_task = tokio::spawn(start_listening(
        channel_pool,
        vec![BLOCKMESH_PG_NOTIFY_WORKER],
        tx.clone(),
        send_to_rx,
    ));
    let db_aggregator_add = tokio::spawn(add_to_aggregates_aggregator(
        joiner_tx.clone(),
        db_pool.clone(),
        tx.subscribe(),
        env::var("ADD_TO_AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));
    let db_aggregator_users_ip_task = tokio::spawn(users_ip_aggregator(
        joiner_tx.clone(),
        db_pool.clone(),
        tx.subscribe(),
        env::var("USERS_IP_AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));
    let db_aggregates_aggregator_task = tokio::spawn(aggregates_aggregator(
        joiner_tx.clone(),
        db_pool.clone(),
        tx.subscribe(),
        env::var("AGG_AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));
    let db_analytics_aggregator_task = tokio::spawn(analytics_aggregator(
        db_pool.clone(),
        tx.subscribe(),
        env::var("ANALYTICS_AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));
    let db_daily_stats_aggregator_task = tokio::spawn(daily_stats_aggregator(
        joiner_tx.clone(),
        db_pool.clone(),
        tx.subscribe(),
        env::var("DAILY_STATS_AGG_SIZE")
            .unwrap_or("300".to_string())
            .parse()
            .unwrap_or(300),
        5,
    ));
    let db_special_task = tokio::spawn(special_worker_loop(db_pool.clone()));

    let router = get_router();
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .nest("/", router)
        .layer(cors)
        .layer(Extension(db_pool.clone()));
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);

    tokio::select! {
        o = db_aggregator_add => panic!("db_aggregator_add exit {:?}", o),
        o = bulk_uptime_bonus_task => panic!("bulk_uptime_bonus_task exit {:?}", o),
        o = bulk_task_bonus_task => panic!("bulk_task_bonus_task exit {:?}", o),
        o = db_special_task => panic!("db_special_task exit {:?}", o),
        o = delete_old_tasks_task => panic!("delete_old_tasks_task exit {:?}", o),
        o = joiner_task => panic!("joiner_task exit {:?}", o),
        o = server_task => panic!("server task exit {:?}", o),
        o = finalize_daily_stats_task => panic!("finalize_daily_stats_task exit {:?}", o),
        o = rpc_worker_task => panic!("rpc_worker_task exit {:?}", o),
        o = db_listen_task => panic!("db_listen_task exit {:?}", o),
        o = db_aggregator_users_ip_task => panic!("db_aggregator_users_ip_task exit {:?}", o),
        o = db_aggregates_aggregator_task => panic!("db_aggregates_aggregator_task exit {:?}", o),
        o = db_analytics_aggregator_task => panic!("db_analytics_aggregator_task exit {:?}", o),
        o = db_daily_stats_aggregator_task => panic!("db_daily_stats_aggregator_task exit {:?}", o)
    }
}
