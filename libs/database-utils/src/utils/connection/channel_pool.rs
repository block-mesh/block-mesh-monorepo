use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tracing::log;

pub async fn channel_pool(database_url_envar_name: Option<String>) -> PgPool {
    let url = database_url_envar_name.unwrap_or("DATABASE_URL".to_string());
    let settings = PgConnectOptions::from_str(&env::var(url).unwrap())
        .unwrap()
        .log_statements(log::LevelFilter::Trace);
    PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(
            env::var("ACQUIRE_TIMEOUT_FOR_CHANNEL")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap_or(5),
        ))
        .min_connections(1)
        .max_connections(
            env::var("MAX_CONNECTIONS_FOR_CHANNEL")
                .unwrap_or("5".to_string())
                .parse()
                .unwrap_or(5),
        )
        .idle_timeout(Duration::from_millis(
            env::var("IDLE_TIMEOUT_FOR_CHANNEL")
                .unwrap_or("600000".to_string())
                .parse()
                .unwrap_or(600000),
        ))
        .max_lifetime(Duration::from_millis(
            env::var("MAX_LIFETIME_FOR_CHANNEL")
                .unwrap_or("1000000000000".to_string())
                .parse()
                .unwrap_or(1000000000000),
        ))
        .test_before_acquire(true)
        .connect_with(settings.clone())
        .await
        .unwrap()
}
