use crate::utils::connection::stale_txn_guard::stale_txn_guard;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tracing::log;

pub async fn dashboard_pool(database_url_envar_name: Option<String>) -> PgPool {
    let url_envar = database_url_envar_name.unwrap_or("DASHBOARD_DATABASE_URL".to_string());
    let db_url = env::var(&url_envar).unwrap_or_else(|_| {
        env::var("WRITE_DATABASE_URL")
            .expect("WRITE_DATABASE_URL must be set when DASHBOARD_DATABASE_URL is not set")
    });
    let settings = PgConnectOptions::from_str(&db_url)
        .unwrap()
        .log_statements(log::LevelFilter::Trace)
        .options([
            (
                "statement_timeout",
                env::var("statement_timeout_dashboard").unwrap_or("10000ms".to_string()),
            ),
            (
                "idle_in_transaction_session_timeout",
                env::var("idle_in_transaction_session_timeout_dashboard")
                    .unwrap_or("3000ms".to_string()),
            ),
            (
                "lock_timeout",
                env::var("lock_timeout_dashboard").unwrap_or("1500ms".to_string()),
            ),
        ])
        .ssl_mode(
            if env::var("APP_ENVIRONMENT").unwrap_or_default() == "production"
                && env::var("APPLY_SSL_REQUIRE").unwrap_or_default() == "true"
            {
                sqlx::postgres::PgSslMode::Require
            } else {
                sqlx::postgres::PgSslMode::default()
            },
        );
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(
            env::var("ACQUIRE_TIMEOUT_DASHBOARD")
                .unwrap_or("10".to_string())
                .parse()
                .unwrap_or(10),
        ))
        .min_connections(1)
        .max_connections(
            env::var("MAX_CONNECTIONS_DASHBOARD")
                .unwrap_or("10".to_string())
                .parse()
                .unwrap_or(10),
        )
        .idle_timeout(Duration::from_millis(
            env::var("IDLE_TIMEOUT_DASHBOARD")
                .unwrap_or("500".to_string())
                .parse()
                .unwrap_or(500),
        ))
        .max_lifetime(Duration::from_millis(
            env::var("MAX_LIFETIME_DASHBOARD")
                .unwrap_or("3600000".to_string())
                .parse()
                .unwrap_or(3600000),
        ))
        .before_acquire(stale_txn_guard("dashboard_pool"))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap()
}
