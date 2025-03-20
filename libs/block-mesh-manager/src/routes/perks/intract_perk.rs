use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::perks::get_user_perks::get_user_perks;
use crate::database::perks::update_user_perk::update_user_perk;
use crate::domain::perk::{Perk, PerkName};
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{IntractIdentityType, PerkResponse};
use block_mesh_common::intract::get_intract_user_details;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use chrono::{Duration, Utc};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;

static RATE_LIMIT: OnceCell<HashMapWithExpiry<String, PerkResponse>> = OnceCell::const_new();

pub async fn add_to_cache(
    email: &str,
    perk: &Perk,
    cache: &HashMapWithExpiry<String, PerkResponse>,
) -> PerkResponse {
    let resp = PerkResponse {
        cached: true,
        name: perk.name.to_string(),
        multiplier: perk.multiplier,
        one_time_bonus: perk.one_time_bonus,
    };
    let app_environment = env::var("APP_ENVIRONMENT").unwrap_or("local".to_string());
    if app_environment != "local" {
        let date = Utc::now() + Duration::milliseconds(480_000);
        cache
            .insert(email.to_string(), resp.clone(), Some(date))
            .await;
    }
    resp
}

#[tracing::instrument(name = "intract_perk", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let email = user.email.to_lowercase();
    let cache = RATE_LIMIT
        .get_or_init(|| async { HashMapWithExpiry::new() })
        .await;
    if let Some(resp) = cache.get(&email).await {
        return Ok(Json(resp).into_response());
    }
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut follower_transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    commit_txn(follower_transaction).await?;
    let resp = get_intract_user_details(&email, &IntractIdentityType::Email).await?;
    let new_data = serde_json::to_value(resp.clone()).map_err(|_| Error::InternalServer)?;
    let mut transaction = create_txn(&pool).await?;
    let user_perks = get_user_perks(&mut transaction, &user.user_id).await?;
    let perk = if let Some(perk) = user_perks.iter().find(|i| i.name == PerkName::Intract) {
        if new_data == perk.data {
            let resp = add_to_cache(&email, &perk, &cache).await;
            return Ok(Json(resp).into_response());
        }
        update_user_perk(
            &mut transaction,
            user.user_id,
            PerkName::Intract,
            new_data,
            perk.data.clone(),
        )
        .await?
    } else {
        let score = block_mesh_common::intract::calc_bonus(new_data.clone())?;
        add_perk_to_user(
            &mut transaction,
            user.user_id,
            PerkName::Intract,
            1.0,
            score,
            new_data,
        )
        .await?
    };
    commit_txn(transaction).await?;
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    dashboard_data_extractor(&pool, &mut follower_transaction, state.clone(), user, true).await?;
    commit_txn(follower_transaction).await?;
    let mut resp = add_to_cache(&email, &perk, &cache).await.clone();
    resp.cached = false;
    Ok(Json(resp).into_response())
}
