use crate::database::api_token::create_api_token::create_api_token;
use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::database::invite_code::get_user_opt_by_invited_code::get_user_opt_by_invited_code;
use crate::database::nonce::create_nonce::create_nonce;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::user::create_user::create_user;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_invited_by::update_user_invited_by;
use crate::domain::nonce::Nonce;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use anyhow::anyhow;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::interface::RegisterForm;
use secret::Secret;
use sqlx::PgPool;
use uuid::Uuid;
use validator::validate_email;

#[tracing::instrument(name = "register_post", skip(form, auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let email = form.email.clone();
    if !validate_email(email) {
        return Ok(Error::redirect(
            400,
            "Invalid email".to_string(),
            "Please check if email you inserted is correct".to_string(),
        ));
    }
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
    create_api_token(&mut transaction, user_id).await?;
    create_invite_code(&mut transaction, user_id, Uuid::new_v4().to_string()).await?;
    create_uptime_report(&mut transaction, user_id).await?;
    if let Some(invite_code) = form.invite_code {
        if !invite_code.is_empty() {
            match get_user_opt_by_invited_code(&mut transaction, invite_code).await? {
                Some(invited_by_user) => {
                    let invited_by_user_id = invited_by_user.user_id;
                    update_user_invited_by(&mut transaction, user_id, invited_by_user_id).await?;
                }
                None => {
                    return Ok(Error::redirect(
                        400,
                        "Invite Code Not Found".to_string(),
                        "Please check if the invite you insert is correct".to_string(),
                    ))
                }
            }
        }
    }
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
