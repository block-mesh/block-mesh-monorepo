use crate::routes::router::get_router;
use axum::extract::Request;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use dashmap::DashMap;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use sentry_tower::NewSentryLayer;
use serde_json::Value;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use std::{env, mem, process};
use tokio::net::TcpListener;

mod database;
mod error;
mod pg_listener;
mod routes;

use crate::pg_listener::start_listening;
use block_mesh_common::constants::{BLOCKMESH_PG_NOTIFY_API, BLOCKMESH_PG_NOTIFY_WORKER};
use block_mesh_common::interfaces::server_api::{CheckTokenResponseMap, GetTokenResponseMap};
use database_utils::utils::connection::get_pg_pool;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

fn main() -> anyhow::Result<()> {
    load_dotenv();
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

    setup_tracing_stdout_only_with_sentry();
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run(sentry_layer).await });
    process::exit(1);
}

async fn run(is_with_sentry: bool) {
    let db_pool = get_pg_pool().await;
    let router = get_router();
    let check_token_map: CheckTokenResponseMap = Arc::new(DashMap::new());
    let get_token_map: GetTokenResponseMap = Arc::new(DashMap::new());
    let cors = CorsLayer::permissive();

    let timeout_layer = env::var("TIMEOUT_LAYER")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);

    let app = Router::new()
        .nest("/", router)
        .layer(Extension(db_pool.clone()))
        .layer(Extension(check_token_map.clone()))
        .layer(Extension(get_token_map.clone()))
        .layer(cors);

    let app = if timeout_layer {
        app.layer(TimeoutLayer::new(Duration::from_millis(
            env::var("REQUEST_TIMEOUT")
                .unwrap_or("3500".to_string())
                .parse()
                .unwrap_or(3500),
        )))
    } else {
        app
    };

    let app = if is_with_sentry {
        app.layer(NewSentryLayer::<Request>::new_from_top())
    } else {
        app
    };
    let port = env::var("PORT").unwrap_or("8001".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    let (tx, _rx) = tokio::sync::broadcast::channel::<Value>(
        env::var("BROADCAST_CHANNEL_SIZE")
            .unwrap_or("5000".to_string())
            .parse()
            .unwrap_or(5000),
    );

    let db_listen_task = tokio::spawn(start_listening(
        db_pool.clone(),
        vec![BLOCKMESH_PG_NOTIFY_API],
        tx.clone(),
        check_token_map,
        get_token_map,
    ));
    let server_task = run_server(listener, app);
    tokio::select! {
        o = server_task => panic!("server task exit {:?}", o),
        o = db_listen_task => panic!("db_listen_task exit {:?}", o),
    }
}

pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}
