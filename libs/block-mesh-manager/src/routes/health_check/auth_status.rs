use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::AuthStatusResponse;
use sqlx::PgPool;

#[tracing::instrument(name = "auth_status", skip(auth))]
pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<AuthStatusResponse>, Error> {
    if let Ok(user) = auth.user.ok_or(Error::UserNotFound) {
        let mut transaction = pool.begin().await.map_err(Error::from)?;
        let db_user = get_user_opt_by_id(&mut transaction, &user.id)
            .await
            .map_err(Error::from)?;
        transaction.commit().await.map_err(Error::from)?;
        if let Some(db_user) = db_user {
            return Ok(Json(AuthStatusResponse {
                email: Some(user.email),
                status_code: 200,
                logged_in: true,
                wallet_address: db_user.wallet_address,
            }));
        }
    }
    return Ok(Json(AuthStatusResponse {
        email: None,
        status_code: 404,
        logged_in: false,
        wallet_address: None,
    }));
}
