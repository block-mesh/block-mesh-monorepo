use axum::{Extension, Json};
use sqlx::PgPool;

use block_mesh_common::interface::{CheckTokenRequest, GetTokenResponse};

use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::domain::api_token::ApiTokenStatus;
use crate::errors::error::Error;

#[tracing::instrument(name = "check_token", skip(body))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = get_user_opt_by_email(&mut transaction, &body.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let api_token =
        get_api_token_by_usr_and_status(&mut transaction, &user.id, ApiTokenStatus::Active)
            .await?
            .ok_or(Error::ApiTokenNotFound)?;
    if *api_token.token.as_ref() != body.api_token {
        return Err(Error::ApiTokenMismatch);
    }
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetTokenResponse {
        api_token: *api_token.token.as_ref(),
    }))
}
