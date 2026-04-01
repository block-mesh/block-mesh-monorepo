use crate::database::user::get_extension_activated_not_sent_users::{
    get_extension_activated_not_sent_users, ExtensionActivatedNotSentUser,
};
use crate::database::user::get_pending_snag_email_reward_users::{
    get_pending_snag_email_reward_users, PendingSnagEmailRewardUser,
};
use crate::database::user::get_wallet_connected_not_sent_users::{
    get_wallet_connected_not_sent_users, WalletConnectedNotSentUser,
};
use crate::database::user::update_extension_activated_sent::update_extension_activated_sent;
use crate::database::user::update_snag_email_reward_state::update_snag_email_reward_state;
use crate::database::user::update_wallet_connected_sent::update_wallet_connected_sent;
use crate::utils::snag::{
    sync_connected_wallet, sync_first_activation, sync_registered_email_reward, SnagConfig,
    SnagEmailRewardOutcome, SnagWalletVerification,
};
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

async fn mark_snag_email_reward_consumed(
    pool: &PgPool,
    user: &PendingSnagEmailRewardUser,
) -> anyhow::Result<()> {
    let mut tx = create_txn(pool)
        .await
        .with_context(|| format!("failed to open transaction for {}", user.user_id))?;
    update_snag_email_reward_state(&mut tx, &user.user_id, false, true)
        .await
        .with_context(|| {
            format!(
                "failed to mark snag email reward consumed for {}",
                user.user_id
            )
        })?;
    commit_txn(tx).await.with_context(|| {
        format!(
            "failed to commit snag email reward consumed for {}",
            user.user_id
        )
    })?;
    Ok(())
}

async fn retry_pending_snag_email_reward_for_user(
    client: &Client,
    snag: &SnagConfig,
    pool: &PgPool,
    user: &PendingSnagEmailRewardUser,
) -> anyhow::Result<()> {
    let outcome = sync_registered_email_reward(
        client.clone(),
        snag.clone(),
        user.user_id,
        user.email.clone(),
        user.wallet_address.clone(),
    )
    .await
    .with_context(|| {
        format!(
            "failed to sync registered email reward for {}",
            user.user_id
        )
    })?;
    if outcome == SnagEmailRewardOutcome::Consumed {
        mark_snag_email_reward_consumed(pool, user).await?;
    }
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

async fn mark_wallet_connected_sent(
    pool: &PgPool,
    user: &WalletConnectedNotSentUser,
) -> anyhow::Result<()> {
    let mut tx = create_txn(pool)
        .await
        .with_context(|| format!("failed to open transaction for {}", user.user_id))?;
    update_wallet_connected_sent(&mut tx, &user.user_id, true)
        .await
        .with_context(|| format!("failed to mark wallet_connected_sent for {}", user.user_id))?;
    commit_txn(tx).await.with_context(|| {
        format!(
            "failed to commit wallet_connected_sent for {}",
            user.user_id
        )
    })?;
    Ok(())
}

async fn retry_pending_wallet_snag_sync_for_user(
    client: &Client,
    snag: &SnagConfig,
    pool: &PgPool,
    user: &WalletConnectedNotSentUser,
) -> anyhow::Result<()> {
    sync_connected_wallet(
        client.clone(),
        snag.clone(),
        user.user_id,
        user.email.clone(),
        user.wallet_address.clone(),
        SnagWalletVerification::verified_locally_only(),
    )
    .await
    .with_context(|| format!("failed to sync connected wallet for {}", user.user_id))?;
    mark_wallet_connected_sent(pool, user).await?;
    Ok(())
}

async fn retry_pending_snag_sync_loop_inner_loop(
    client: &Client,
    snag: &SnagConfig,
    pool: &PgPool,
    batch_size: i64,
    call_sleep_ms: u64,
) -> anyhow::Result<()> {
    let email_reward_users = get_pending_snag_email_reward_users(pool, batch_size)
        .await
        .context("failed to load pending snag email reward retry users")?;
    if !email_reward_users.is_empty() {
        tracing::info!(
            "retrying Snag email reward sync for {} pending users",
            email_reward_users.len()
        );
    }

    for user in email_reward_users {
        if let Err(error) =
            retry_pending_snag_email_reward_for_user(client, snag, pool, &user).await
        {
            tracing::warn!(
                "failed to retry pending snag email reward sync for {}: {}",
                user.user_id,
                error
            );
        }

        tokio::time::sleep(Duration::from_millis(call_sleep_ms)).await;
    }

    let extension_users = get_extension_activated_not_sent_users(pool, batch_size)
        .await
        .context("failed to load pending extension Snag retry users")?;
    if !extension_users.is_empty() {
        tracing::info!(
            "retrying Snag extension sync for {} pending users",
            extension_users.len()
        );
    }

    for user in extension_users {
        if let Err(error) = retry_pending_snag_sync_for_user(client, snag, pool, &user).await {
            tracing::warn!(
                "failed to retry pending extension Snag sync for {}: {}",
                user.user_id,
                error
            );
        }

        tokio::time::sleep(Duration::from_millis(call_sleep_ms)).await;
    }

    let wallet_users = get_wallet_connected_not_sent_users(pool, batch_size)
        .await
        .context("failed to load pending wallet Snag retry users")?;
    if !wallet_users.is_empty() {
        tracing::info!(
            "retrying Snag wallet sync for {} pending users",
            wallet_users.len()
        );
    }

    for user in wallet_users {
        if let Err(error) = retry_pending_wallet_snag_sync_for_user(client, snag, pool, &user).await
        {
            tracing::warn!(
                "failed to retry pending wallet Snag sync for {}: {}",
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
