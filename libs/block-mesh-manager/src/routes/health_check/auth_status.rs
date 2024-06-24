use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::AuthStatusResponse;

#[tracing::instrument(name = "auth_status", skip(auth))]
pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<AuthStatusResponse>, Error> {
    Ok(Json(match auth.user.ok_or(Error::UserNotFound) {
        Ok(user) => AuthStatusResponse {
            email: Some(user.email),
            status_code: 200,
            logged_in: true,
        },
        Err(_) => AuthStatusResponse {
            email: None,
            status_code: 404,
            logged_in: false,
        },
    }))
}
