use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;

#[tracing::instrument(name = "logout", skip(auth))]
pub async fn handler(
    Extension(mut auth): Extension<AuthSession<Backend>>,
) -> Result<Redirect, Error> {
    auth.logout()
        .await
        .map_err(|e| Error::Auth(e.to_string()))?;
    Ok(Redirect::to("/login"))
}
