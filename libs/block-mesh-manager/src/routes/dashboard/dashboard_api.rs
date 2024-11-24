use axum::extract::State;
use axum::{Extension, Json};
use sqlx::PgPool;
use std::sync::Arc;
#[allow(unused_imports)]
use tracing::Level;

use crate::errors::error::Error;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;
use crate::startup::application::AppState;
use block_mesh_common::interfaces::server_api::{DashboardRequest, DashboardResponse};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};

#[tracing::instrument(name = "dashboard_api", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DashboardRequest>,
) -> Result<Json<DashboardResponse>, Error> {
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut follower_transaction, &body.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(follower_transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    let data =
        dashboard_data_extractor(&pool, &mut follower_transaction, state.clone(), user).await?;
    commit_txn(follower_transaction).await?;
    Ok(Json(data))
}
