use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    GetEmailViaTokenRequest, GetEmailViaTokenResponse,
};
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use sqlx::PgPool;

#[tracing::instrument(name = "get_email_via_token", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetEmailViaTokenRequest>,
) -> Result<Json<GetEmailViaTokenResponse>, Error> {
    let mut transaction = pool.begin().await?;
    let token = body.token;
    let nonce = get_nonce_by_nonce(&mut transaction, &token)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &nonce.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    transaction.commit().await?;
    Ok(Json(GetEmailViaTokenResponse { email: user.email }))
}
