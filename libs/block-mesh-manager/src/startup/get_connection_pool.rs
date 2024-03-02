use crate::configuration::database_settings::DatabaseSettings;
use crate::domain::secret::Secret;
use anyhow::anyhow;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, PgPool};
use std::str::FromStr;
use std::time::Duration;
use std::{thread, time};
use tracing::log;

#[tracing::instrument(name = "get_connection_pool", skip(settings, database_url), ret, err)]
pub async fn get_connection_pool(
    settings: &DatabaseSettings,
    database_url: Option<&Secret<String>>,
) -> Result<PgPool, anyhow::Error> {
    let mut retries = 1;
    let settings: PgConnectOptions = match database_url {
        None => settings.with_db(),
        Some(database_url) => PgConnectOptions::from_str(database_url.as_ref())?
            .log_statements(log::LevelFilter::Trace)
            .clone(),
    };

    loop {
        tracing::info!("attempting to connect to database - retry : {}", retries);
        if retries > 10 {
            return Err(anyhow!(
                "failed to connect to database after retries : {}",
                retries
            ));
        }
        let pool_connection = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(5))
            .max_connections(100)
            .connect_with(settings.clone())
            .await;
        match pool_connection {
            Ok(pool) => {
                tracing::info!("connected to database - retry : {}", retries);
                return Ok(pool);
            }
            Err(error) => {
                tracing::warn!(
                    "failed to connect to database - retry : {} - error {:#?}",
                    retries,
                    error
                );
                let ten_millis = time::Duration::from_millis(1_000);
                thread::sleep(ten_millis);
                retries += 1;
            }
        }
    }
}
