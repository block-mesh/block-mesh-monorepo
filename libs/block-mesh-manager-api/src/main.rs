use crate::routes::check_token::CheckTokenResponseMap;
use crate::routes::get_token::GetTokenResponseMap;
use crate::routes::router::get_router;
use axum::extract::Request;
use axum::{Extension, Router};
use block_mesh_common::env::load_dotenv::load_dotenv;
use dashmap::DashMap;
use logger_general::tracing::setup_tracing_stdout_only_with_sentry;
use sentry_tower::NewSentryLayer;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::{env, mem};
use tokio::net::TcpListener;

mod database;
mod error;
mod routes;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tracing::log;

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
    Ok(())
}

async fn run(is_with_sentry: bool) {
    let settings = PgConnectOptions::from_str(&env::var("DATABASE_URL").unwrap())
        .unwrap()
        .log_statements(log::LevelFilter::Trace)
        .options([
            (
                "statement_timeout",
                env::var("statement_timeout").unwrap_or("0".to_string()),
            ),
            (
                "idle_in_transaction_session_timeout",
                env::var("idle_in_transaction_session_timeout").unwrap_or("3000ms".to_string()),
            ),
        ]);
    let db_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(
            env::var("ACQUIRE_TIMEOUT")
                .unwrap_or("35".to_string())
                .parse()
                .unwrap_or(35),
        ))
        .min_connections(1)
        .max_connections(
            env::var("MAX_CONNECTIONS")
                .unwrap_or("35".to_string())
                .parse()
                .unwrap_or(35),
        )
        .idle_timeout(Duration::from_millis(
            env::var("IDLE_TIMEOUT")
                .unwrap_or("500".to_string())
                .parse()
                .unwrap_or(500),
        ))
        .max_lifetime(Duration::from_millis(
            env::var("MAX_LIFETIME")
                .unwrap_or("30000".to_string())
                .parse()
                .unwrap_or(30000),
        ))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap();
    let router = get_router();
    let check_token_map: CheckTokenResponseMap = Arc::new(DashMap::new());
    let get_token_map: GetTokenResponseMap = Arc::new(DashMap::new());
    let cors = CorsLayer::permissive();
    let app = Router::new()
        .nest("/", router)
        .layer(Extension(db_pool.clone()))
        .layer(Extension(check_token_map))
        .layer(Extension(get_token_map))
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_millis(
            env::var("REQUEST_TIMEOUT")
                .unwrap_or("1000".to_string())
                .parse()
                .unwrap_or(1000),
        )));
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

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
