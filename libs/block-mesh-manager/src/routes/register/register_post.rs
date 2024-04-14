use crate::database::nonce::create_nonce::create_nonce;
use crate::database::user::create_user::create_user;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::domain::nonce::Nonce;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use anyhow::anyhow;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use bcrypt::{hash, DEFAULT_COST};
use secret::Secret;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterForm {
    email: String,
    password: String,
    password_confirm: String,
}

#[tracing::instrument(name = "register_post", skip(form, auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    if form.password_confirm != form.password {
        return Err(Error::PasswordMismatch);
    }
    let user = get_user_opt_by_email(&mut transaction, &form.email).await?;
    if user.is_some() {
        return Err(Error::UserAlreadyExists);
    }
    let nonce = Nonce::generate_nonce(16);
    let nonce_secret = Secret::from(nonce.clone());
    let hashed_password = hash(form.password.clone(), DEFAULT_COST)?;
    let user_id = create_user(&mut transaction, None, &form.email, &hashed_password)
        .await
        .map_err(Error::from)?;
    create_nonce(&mut transaction, &user_id, &nonce_secret).await?;
    transaction.commit().await.map_err(Error::from)?;

    let creds: Credentials = Credentials {
        email: form.email,
        password: Secret::from(form.password),
        nonce,
    };
    let session = auth
        .authenticate(creds)
        .await
        .map_err(|_| Error::Auth(anyhow!("Authentication failed").to_string()))?
        .ok_or(Error::UserNotFound)?;
    auth.login(&session)
        .await
        .map_err(|_| Error::Auth(anyhow!("Login failed").to_string()))?;
    Ok(Redirect::to("/dashboard"))
}
