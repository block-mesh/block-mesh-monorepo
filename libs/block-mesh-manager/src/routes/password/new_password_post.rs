use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id_pool;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_password::update_user_password;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use axum::response::Redirect;
use axum::{Extension, Form};
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::interfaces::server_api::NewPasswordForm;
use block_mesh_common::routes_enum::RoutesEnum;
use sqlx::PgPool;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<NewPasswordForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let email = form.email.clone().to_ascii_lowercase();
    if form.password_confirm != form.password {
        return Ok(Error::redirect(
            400,
            "Password Mismatch",
            "Please check if your password and password confirm are the same",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id_pool(&pool, &user.id)
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
        RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
    ))
}
