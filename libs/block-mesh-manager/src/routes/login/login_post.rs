use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use anyhow::anyhow;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::LoginForm;
use secret::Secret;
use sqlx::PgPool;

#[tracing::instrument(name = "login_post", skip(form, auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = get_user_opt_by_email(&mut transaction, &form.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let creds: Credentials = Credentials {
        email: form.email,
        password: Secret::from(form.password),
        nonce: nonce.nonce.as_ref().to_string(),
    };
    let session = auth
        .authenticate(creds)
        .await
        .map_err(|_| Error::Auth(anyhow!("Authentication failed").to_string()))?
        .ok_or(Error::UserNotFound)?;
    auth.login(&session)
        .await
        .map_err(|_| Error::Auth(anyhow!("Login failed").to_string()))?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Redirect::to("/dashboard"))
}
