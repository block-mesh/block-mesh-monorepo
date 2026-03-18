use crate::database::user::get_extension_activated_not_sent_users::{
    get_extension_activated_not_sent_users, ExtensionActivatedNotSentUser,
};
use crate::database::user::update_extension_activated_sent::update_extension_activated_sent;
use crate::utils::snag::{sync_first_activation, SnagConfig};
use anyhow::Context;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::Client;
use sqlx::PgPool;
use std::env;
use std::time::Duration;

async fn mark_extension_activated_sent(
    pool: &PgPool,
    user: &ExtensionActivatedNotSentUser,
) -> anyhow::Result<()> {
    let mut tx = create_txn(pool)
        .await
        .with_context(|| format!("failed to open transaction for {}", user.user_id))?;
    update_extension_activated_sent(&mut tx, &user.user_id, true)
        .await
        .with_context(|| {
            format!(
                "failed to mark extension_activated_sent for {}",
                user.user_id
            )
        })?;
    commit_txn(tx).await.with_context(|| {
        format!(
            "failed to commit extension_activated_sent for {}",
            user.user_id
        )
    })?;
    Ok(())
}

async fn retry_pending_snag_sync_for_user(
    client: &Client,
    snag: &SnagConfig,
    pool: &PgPool,
    user: &ExtensionActivatedNotSentUser,
) -> anyhow::Result<()> {
    sync_first_activation(
        client.clone(),
        snag.clone(),
        user.user_id,
        user.email.clone(),
        user.wallet_address.clone(),
    )
    .await
    .with_context(|| format!("failed to sync first activation for {}", user.user_id))?;
    mark_extension_activated_sent(pool, user).await?;
    Ok(())
}

async fn retry_pending_snag_sync_loop_inner_loop(
    client: &Client,
    snag: &SnagConfig,
    pool: &PgPool,
    batch_size: i64,
    call_sleep_ms: u64,
) -> anyhow::Result<()> {
    let users = get_extension_activated_not_sent_users(pool, batch_size)
        .await
        .context("failed to load pending Snag retry users")?;
    if !users.is_empty() {
        tracing::info!("retrying Snag sync for {} pending users", users.len());
    }

    for user in users {
        if let Err(error) = retry_pending_snag_sync_for_user(client, snag, pool, &user).await {
            tracing::warn!(
                "failed to retry pending Snag sync for {}: {}",
                user.user_id,
                error
            );
        }

        tokio::time::sleep(Duration::from_millis(call_sleep_ms)).await;
    }

    Ok(())
}

#[tracing::instrument(name = "retry_pending_snag_sync_loop", skip_all)]
pub async fn retry_pending_snag_sync_loop(client: Client, snag: SnagConfig, pool: PgPool) {
    let iteration_sleep_ms = env::var("SNAG_RETRY_LOOP_SLEEP_MS")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(60_000_u64);
    let call_sleep_ms = env::var("SNAG_RETRY_CALL_SLEEP_MS")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(500_u64);
    let batch_size = env::var("SNAG_RETRY_BATCH_SIZE")
        .ok()
        .and_then(|var| var.parse().ok())
        .unwrap_or(100_i64)
        .max(1);

    loop {
        if let Err(error) = retry_pending_snag_sync_loop_inner_loop(
            &client,
            &snag,
            &pool,
            batch_size,
            call_sleep_ms,
        )
        .await
        {
            tracing::warn!("{error}");
        }

        tokio::time::sleep(Duration::from_millis(iteration_sleep_ms)).await;
    }
}
