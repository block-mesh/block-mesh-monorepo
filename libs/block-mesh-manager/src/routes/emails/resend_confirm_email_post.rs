use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use block_mesh_common::interfaces::server_api::ResendConfirmEmailForm;
use block_mesh_common::routes_enum::RoutesEnum;
use sqlx::PgPool;
use std::sync::Arc;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<ResendConfirmEmailForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await?;
    let user = get_user_opt_by_email(&mut transaction, &form.email.to_ascii_lowercase())
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let _ = state
        .email_client
        .send_confirmation_email_aws(&user.email, nonce.nonce.expose_secret())
        .await;
    transaction.commit().await?;
    Ok(NotificationRedirect::redirect(
        "Email Sent",
        "Please check your email",
        RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
    ))
}
