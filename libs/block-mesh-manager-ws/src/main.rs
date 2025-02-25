#[allow(unused_imports)]
use anyhow::Context;
#[allow(unused_imports)]
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use block_mesh_common::env::load_dotenv::load_dotenv;
use block_mesh_common::interfaces::db_messages::DBMessage;
use block_mesh_manager_ws::app::app;
use block_mesh_manager_ws::get_pending_twitter_tasks_loop::get_pending_twitter_tasks_loop;
use block_mesh_manager_ws::joiner_loop::joiner_loop;
use block_mesh_manager_ws::message_aggregator::collect_messages;
use block_mesh_manager_ws::redis_loop::redis_loop;
use block_mesh_manager_ws::state::WsAppState;
use logger_general::tracing::{get_sentry_layer, setup_tracing_stdout_only_with_sentry};
use std::sync::Arc;
#[allow(unused_imports)]
use std::time::Duration;
use std::{env, mem, process};
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
#[allow(unused_imports)]
use uuid::Uuid;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let sentry_layer = get_sentry_layer();
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
    let (joiner_tx, joiner_rx) = flume::bounded::<JoinHandle<()>>(10_000);
    let state = Arc::new(WsAppState::new(tx, joiner_tx.clone()).await);
    state.init_redis().await;
    let channel_pool = state.channel_pool.clone();
    let get_pending_twitter_tasks_loop_task =
        tokio::spawn(get_pending_twitter_tasks_loop(state.clone()));
    let server_task = app(listener, state.clone());

    let joiner_task = tokio::spawn(joiner_loop(joiner_rx));
    let redis_task = tokio::spawn(redis_loop(state.clone()));
    let collect_messages_task = tokio::spawn(collect_messages(
        joiner_tx,
        rx,
        channel_pool,
        env::var("CHANNEL_AGG_SIZE")
            .unwrap_or("10".to_string())
            .parse()
            .unwrap_or(10),
        5,
    ));
    tokio::select! {
        o = get_pending_twitter_tasks_loop_task => panic!("get_pending_twitter_tasks_loop_task {:?}", o),
        o = redis_task => panic!("redis_task {:?}", o),
        o = joiner_task => panic!("joiner_task {:?}", o),
        o = server_task => panic!("server_task {:?}", o),
        o = collect_messages_task => panic!("collect_messages_task {:?}", o)
    }
}
