use axum::{Extension, Json};
use sqlx::PgPool;

use block_mesh_common::interfaces::server_api::{
    GetLatestInviteCodeRequest, GetLatestInviteCodeResponse,
};

use crate::database::api_token::find_token::find_token;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;

#[tracing::instrument(name = "get_latest_invite_code", skip(pool, body), fields(email = body.email), err, ret, level = "trace")]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetLatestInviteCodeRequest>,
) -> Result<Json<GetLatestInviteCodeResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email != body.email {
        return Err(Error::UserNotFound);
    }
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetLatestInviteCodeResponse {
        invite_code: user_invite_code.invite_code,
    }))
}
