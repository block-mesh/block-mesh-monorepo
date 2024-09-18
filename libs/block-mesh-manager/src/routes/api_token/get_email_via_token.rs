use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce_pool;
use crate::database::user::get_user_by_id::get_user_opt_by_id_pool;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    GetEmailViaTokenRequest, GetEmailViaTokenResponse,
};
use sqlx::PgPool;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetEmailViaTokenRequest>,
) -> Result<Json<GetEmailViaTokenResponse>, Error> {
    let token = body.token;
    let nonce = get_nonce_by_nonce_pool(&pool, &token)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let user = get_user_opt_by_id_pool(&pool, &nonce.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    Ok(Json(GetEmailViaTokenResponse { email: user.email }))
}
