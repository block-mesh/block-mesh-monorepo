use crate::database::notify::notify_worker::notify_worker;
use crate::database::users_ip::update_users_ip_bulk::update_users_ip_bulk;
use crate::startup::application::AppState;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::db_messages::UsersIpMessage;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use chrono::Utc;
use flume::Receiver;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "users_ip_agg", skip_all)]
pub async fn users_ip_agg(
    pool: PgPool,
    rx: Receiver<UsersIpMessage>,
    state: Arc<AppState>,
) -> Result<(), anyhow::Error> {
    while let Ok(message) = rx.recv_async().await {
        let _ = notify_worker(&pool, message.clone()).await;
    }
    Ok(())
}
