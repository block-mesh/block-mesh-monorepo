#[allow(unused_imports)]
use anyhow::Context;
#[allow(unused_imports)]
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_common::constants::BLOCKMESH_WS_REDIS_COUNT_KEY;
use block_mesh_common::env::load_dotenv::load_dotenv;
use block_mesh_common::interfaces::db_messages::DBMessage;
use block_mesh_manager_ws::app::app;
use block_mesh_manager_ws::message_aggregator::collect_messages;
use block_mesh_manager_ws::state::AppState;
#[allow(unused_imports)]
use block_mesh_manager_ws::websocket::settings_loop::settings_loop;
use block_mesh_manager_ws::websocket::ws_base_msg_loop::ws_base_msg_loop;
use block_mesh_manager_ws::websocket::ws_bulk_loop::ws_bulk_loop;
use block_mesh_manager_ws::websocket::ws_keep_alive::ws_keep_alive;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use redis::{AsyncCommands, RedisResult};
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::Duration;
use std::{env, mem, process};
use tokio::net::TcpListener;
#[allow(unused_imports)]
use uuid::Uuid;

fn main() {
    let sentry_layer = env::var("SENTRY_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let sentry_url = env::var("SENTRY_WS").unwrap_or_default();
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

async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    let port = env::var("PORT")
        .unwrap_or("8002".to_string())
        .parse()
        .unwrap_or(8002);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let (tx, rx) = flume::bounded::<DBMessage>(10_000);
    let state = Arc::new(AppState::new(tx).await);
    let channel_pool = state.channel_pool.clone();
    let mut redis = state.redis.clone();
    let _: RedisResult<()> = redis.set(BLOCKMESH_WS_REDIS_COUNT_KEY, 0).await;
    let broadcaster = state.websocket_manager.broadcaster.clone();
    let p = state.pool.clone();
    let b = broadcaster.clone();
    let ws_bulk_loop_task = tokio::spawn(ws_bulk_loop(p, b));
    let ping_task = tokio::spawn(ws_keep_alive(broadcaster.clone()));
    let b = broadcaster.clone();
    let base_msg_task = tokio::spawn(ws_base_msg_loop(b));
    let server_task = app(listener, state);
    let collect_messages_task = tokio::spawn(collect_messages(
        rx,
        channel_pool,
        env::var("CHANNEL_AGG_SIZE")
            .unwrap_or("10".to_string())
            .parse()
            .unwrap_or(10),
        5,
    ));
    tokio::select! {
        o = base_msg_task => panic!("base_msg_task {:?}", o),
        o = ping_task => panic!("ping_task {:?}", o),
        o = server_task => panic!("server_task {:?}", o),
        o = ws_bulk_loop_task => panic!("ws_bulk_loop_task {:?}", o),
        o = collect_messages_task => panic!("collect_messages_task {:?}", o)
    }
}
