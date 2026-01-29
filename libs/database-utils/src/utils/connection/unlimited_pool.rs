use crate::utils::connection::stale_txn_guard::stale_txn_guard;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tracing::log;

pub async fn unlimited_pool(database_url_envar_name: Option<String>) -> PgPool {
    let url = database_url_envar_name.unwrap_or("UNLIMITED_DATABASE_URL".to_string());
    let settings = PgConnectOptions::from_str(&env::var(url).unwrap())
        .unwrap()
        .log_statements(log::LevelFilter::Trace)
        .options([
            (
                "statement_timeout",
                env::var("statement_timeout_unlimited").unwrap_or("0".to_string()),
            ),
            (
                "idle_in_transaction_session_timeout",
                env::var("idle_in_transaction_session_timeout_unlimited")
                    .unwrap_or("3000ms".to_string()),
            ),
            (
                "lock_timeout",
                env::var("lock_timeout_unlimited").unwrap_or("1500ms".to_string()),
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
            env::var("ACQUIRE_TIMEOUT_UNLIMITED")
                .unwrap_or("35".to_string())
                .parse()
                .unwrap_or(35),
        ))
        .min_connections(1)
        .max_connections(
            env::var("MAX_CONNECTIONS_UNLIMITED")
                .unwrap_or("35".to_string())
                .parse()
                .unwrap_or(35),
        )
        .idle_timeout(Duration::from_millis(
            env::var("IDLE_TIMEOUT_UNLIMITED")
                .unwrap_or("500".to_string())
                .parse()
                .unwrap_or(500),
        ))
        .max_lifetime(Duration::from_millis(
            env::var("MAX_LIFETIME_UNLIMITED")
                .unwrap_or("3600000".to_string())
                .parse()
                .unwrap_or(3600000),
        ))
        .before_acquire(stale_txn_guard("unlimited_pool"))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap()
}
