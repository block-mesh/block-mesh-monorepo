use axum::{Extension, Json};
use sqlx::PgPool;

use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::errors::error::Error;
use block_mesh_common::interfaces::server_api::{
    GetLatestInviteCodeRequest, GetLatestInviteCodeResponse,
};
use block_mesh_manager_database_domain::domain::find_token::find_token;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};

#[tracing::instrument(name = "get_latest_invite_code", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetLatestInviteCodeRequest>,
) -> Result<Json<GetLatestInviteCodeResponse>, Error> {
    let mut transaction = create_txn(&pool).await?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        commit_txn(transaction).await?;
        return Err(Error::UserNotFound);
    }
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    commit_txn(transaction).await?;
    Ok(Json(GetLatestInviteCodeResponse {
        invite_code: user_invite_code.invite_code,
    }))
}
