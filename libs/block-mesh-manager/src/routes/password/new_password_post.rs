use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_password::update_user_password;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use axum::response::Redirect;
use axum::{Extension, Form};
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::interfaces::server_api::NewPasswordForm;
use sqlx::PgPool;

#[tracing::instrument(name = "new_password_post", skip(form, pool))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<NewPasswordForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    if form.password_confirm != form.password {
        return Ok(Error::redirect(
            400,
            "Password Mismatch",
            "Please check if your password and password confirm are the same",
            "/register",
        ));
    }
    let user = get_user_opt_by_email(&mut transaction, &form.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    if *nonce.nonce.expose_secret() != form.token {
        return Err(Error::TokenMismatch);
    }
    let hashed_password = hash(form.password.clone(), DEFAULT_COST)?;
    update_user_password(&mut transaction, user.id, hashed_password).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(NotificationRedirect::redirect(
        "Password updated",
        "Please use the new password and login",
        "/login",
    ))
}
