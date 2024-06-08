use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use block_mesh_common::interfaces::server_api::ResendConfirmEmailForm;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "resend_confirm_email_post", skip(form, state))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<ResendConfirmEmailForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = get_user_opt_by_email(&mut transaction, &form.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    state
        .email_client
        .send_confirmation_email(&form.email, nonce.nonce.expose_secret())
        .await;
    transaction.commit().await.map_err(Error::from)?;
    Ok(NotificationRedirect::redirect(
        "Email Sent",
        "Please check your email",
        "/login",
    ))
}
