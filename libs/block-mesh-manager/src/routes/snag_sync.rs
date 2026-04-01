use crate::database::user::update_snag_email_reward_state::update_snag_email_reward_state;
use crate::startup::application::AppState;
use crate::utils::snag::{sync_registered_email_reward, SnagEmailRewardOutcome};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub fn spawn_snag_email_reward_sync(
    state: Arc<AppState>,
    pool: PgPool,
    user_id: Uuid,
    email: String,
    wallet_address: Option<String>,
) {
    let client = state.client.clone();
    let snag = state.snag.clone();
    tokio::spawn(async move {
        match sync_registered_email_reward(client, snag, user_id, email, wallet_address).await {
            Ok(SnagEmailRewardOutcome::Consumed) => {
                if let Ok(mut tx) = create_txn(&pool).await {
                    if let Err(error) =
                        update_snag_email_reward_state(&mut tx, &user_id, false, true).await
                    {
                        tracing::warn!(
                            "failed to mark snag email reward consumed for {user_id}: {error}"
                        );
                    } else if let Err(error) = commit_txn(tx).await {
                        tracing::warn!(
                            "failed to commit snag email reward consumed for {user_id}: {error}"
                        );
                    }
                }
            }
            Ok(SnagEmailRewardOutcome::Pending) => {}
            Err(error) => {
                tracing::warn!(
                    "failed to sync snag registered email reward for {user_id}: {error}"
                );
            }
        }
    });
}
