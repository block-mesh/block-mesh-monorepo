use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use block_mesh_common::interfaces::server_api::ResetPasswordForm;
use chrono::{Duration, Utc};
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::HeaderMap;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::OnceCell;

static RATE_LIMIT_EMAIL: OnceCell<DashSetWithExpiry<String>> = OnceCell::const_new();

pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<ResetPasswordForm>,
) -> Result<Redirect, Error> {
    let email = form.email.clone().to_ascii_lowercase();
    let app_env = get_envar("APP_ENVIRONMENT").await;
    let header_ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    }
    .to_string();
    let cache = RATE_LIMIT_EMAIL
        .get_or_init(|| async { DashSetWithExpiry::new() })
        .await;
    if cache.get(&email).is_some() || cache.get(&header_ip).is_some() {
        return Err(Error::NotAllowedRateLimit);
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    cache.insert(email, Some(date));
    cache.insert(header_ip, Some(date));
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let email_mode = get_envar("EMAIL_MODE").await;
    if email_mode == "AWS" {
        state
            .email_client
            .send_reset_password_email_aws(&user.email, nonce.nonce.expose_secret())
            .await?;
    } else {
        state
            .email_client
            .send_reset_password_email_gmail(&user.email, nonce.nonce.expose_secret())
            .await?;
    }
    commit_txn(transaction).await?;
    Ok(NotificationRedirect::redirect(
        "Email Sent",
        "Please check your email",
        "/login",
    ))
}
