use axum::extract::State;
use axum::Json;
use std::sync::Arc;

use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use block_mesh_common::interfaces::server_api::{
    GetLatestInviteCodeRequest, GetLatestInviteCodeResponse,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};

#[tracing::instrument(name = "get_latest_invite_code", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetLatestInviteCodeRequest>,
) -> Result<Json<GetLatestInviteCodeResponse>, Error> {
    let mut transaction = create_txn(&state.follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &body.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    let user_invite_code = get_user_latest_invite_code(&mut transaction, &user.user_id)
        .await
        .map_err(Error::from)?;
    commit_txn(transaction).await?;
    Ok(Json(GetLatestInviteCodeResponse {
        invite_code: user_invite_code.invite_code,
    }))
}
