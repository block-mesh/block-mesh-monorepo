use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use std::env;
use tracing::log;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
    #[serde(default)]
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Disable
        };

        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .ssl_mode(ssl_mode)
            .username(&self.username)
            .password(&self.password)
            .options([
                (
                    "statement_timeout",
                    env::var("statement_timeout").unwrap_or("0".to_string()),
                ),
                (
                    "idle_in_transaction_session_timeout",
                    env::var("idle_in_transaction_session_timeout").unwrap_or("3000ms".to_string()),
                ),
            ])
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.name);
        options.clone().log_statements(log::LevelFilter::Trace);
        options
    }
}
