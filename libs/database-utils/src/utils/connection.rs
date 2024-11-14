use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tracing::log;

pub async fn get_pg_pool(database_url_envar_name: Option<String>) -> PgPool {
    let url = database_url_envar_name.unwrap_or("DATABASE_URL".to_string());
    let settings = PgConnectOptions::from_str(&env::var(url).unwrap())
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
            (
                "lock_timeout",
                env::var("lock_timeout").unwrap_or("1500ms".to_string()),
            ),
        ]);
    PgPoolOptions::new()
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
                .unwrap_or("360000".to_string())
                .parse()
                .unwrap_or(360000),
        ))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap()
}

pub async fn get_unlimited_pg_pool(database_url_envar_name: Option<String>) -> PgPool {
    let url = database_url_envar_name.unwrap_or("DATABASE_URL".to_string());
    let settings = PgConnectOptions::from_str(&env::var(url).unwrap())
        .unwrap()
        .log_statements(log::LevelFilter::Trace);
    PgPoolOptions::new()
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
                .unwrap_or("3600000".to_string())
                .parse()
                .unwrap_or(3600000),
        ))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap()
}
