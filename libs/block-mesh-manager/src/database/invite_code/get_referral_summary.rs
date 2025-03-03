use block_mesh_common::interfaces::server_api::{ReferralSummary, TmpReferralSummary};
use chrono::{Duration, Utc};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;

static CACHE: OnceCell<Arc<RwLock<HashMapWithExpiry<Uuid, ReferralSummary>>>> =
    OnceCell::const_new();

pub async fn get_user_referrals_summary(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<ReferralSummary> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMapWithExpiry::new())) })
        .await;
    if let Some(out) = cache.read().await.get(user_id).await {
        return Ok(out);
    }
    let summary: TmpReferralSummary = sqlx::query_as!(
        TmpReferralSummary,
        r#"
        SELECT
           COUNT(*) as total_invites,
           COUNT(*) FILTER (WHERE verified_email = TRUE) AS total_verified_email,
           COUNT(*) FILTER (WHERE proof_of_humanity = TRUE) AS total_verified_human,
           COUNT(*) FILTER (WHERE proof_of_humanity = TRUE AND verified_email = TRUE) AS total_eligible
        FROM users
        WHERE invited_by = $1
        "#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    let output = ReferralSummary {
        total_invites: summary.total_invites.unwrap_or_default(),
        total_verified_email: summary.total_verified_email.unwrap_or_default(),
        total_verified_human: summary.total_verified_human.unwrap_or_default(),
        total_eligible: summary.total_eligible.unwrap_or_default(),
    };
    let date = Utc::now() + Duration::milliseconds(600_000);
    cache
        .write()
        .await
        .insert(user_id.clone(), output.clone(), Some(date))
        .await;
    Ok(output)
}
