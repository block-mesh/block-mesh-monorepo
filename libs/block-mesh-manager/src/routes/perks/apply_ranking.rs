use crate::database::invite_code::get_referral_summary::get_user_referrals_summary;
use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::RankBonus;
use chrono::{Duration, Utc};
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;

static RATE_USER: OnceCell<DashSetWithExpiry<Uuid>> = OnceCell::const_new();

#[tracing::instrument(name = "apply_ranking", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let cache = RATE_USER
        .get_or_init(|| async { DashSetWithExpiry::new() })
        .await;
    if cache.get(&user.id).is_some() {
        return Ok((StatusCode::TOO_MANY_REQUESTS, "Rate limited").into_response());
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    cache.insert(user.id, Some(date));
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let referral_summary = get_user_referrals_summary(&mut follower_transaction, &user.id)
        .await
        .map_err(Error::from)?;
    commit_txn(follower_transaction).await?;
    if referral_summary.total_eligible > 25 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Novice,
            1.0,
            RankBonus::Novice.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 50 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Apprentice,
            1.0,
            RankBonus::Apprentice.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 100 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Journeyman,
            1.0,
            RankBonus::Journeyman.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 200 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Expert,
            1.0,
            RankBonus::Expert.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 500 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Master,
            1.0,
            RankBonus::Master.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 750 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Grandmaster,
            1.0,
            RankBonus::Grandmaster.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    if referral_summary.total_eligible > 1_000 {
        let mut transaction = create_txn(&pool).await?;
        let _ = add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Legend,
            1.0,
            RankBonus::Legend.into(),
            serde_json::from_str("{}").unwrap(),
        )
        .await;
        let _ = commit_txn(transaction).await;
    }
    Ok((StatusCode::OK, "OK").into_response())
}
