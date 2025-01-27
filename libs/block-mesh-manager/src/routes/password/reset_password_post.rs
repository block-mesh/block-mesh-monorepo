use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::spam_email::get_spam_emails::{
    get_email_rate_limit, get_spam_emails_cache, update_email_rate_limit,
};
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::domain::spam_email::SpamEmail;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use block_mesh_common::constants::{DeviceType, BLOCK_MESH_EMAILS};
use block_mesh_common::interfaces::server_api::{ResetPasswordForm, SendEmail};
use block_mesh_common::reqwest::http_client;
use block_mesh_common::routes_enum::RoutesEnum;
use chrono::{Duration, Utc};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::HeaderMap;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<ResetPasswordForm>,
) -> Result<Redirect, Error> {
    let email = form.email.clone().to_ascii_lowercase();
    let spam_emails = get_spam_emails_cache().await;
    let email_domain = match email.split('@').last() {
        Some(d) => d.to_string(),
        None => {
            return Ok(Error::redirect(
                400,
                "Invalid email domain",
                "Please check if email you inserted is correct",
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ));
        }
    };

    if SpamEmail::check_domains(&email_domain, spam_emails).is_err() {
        return Ok(Error::redirect(
            400,
            "Invalid email domain",
            "Please check if email you inserted is correct",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }
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

    let cache = get_email_rate_limit().await;
    if cache.get(&email).await.is_some() || cache.get(&header_ip).await.is_some() {
        return Err(Error::NotAllowedRateLimit);
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    update_email_rate_limit(&email, Some(date)).await;
    update_email_rate_limit(&header_ip, Some(date)).await;

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
    } else if email_mode == "SERVICE" {
        let client = http_client(DeviceType::AppServer);
        client
            .get(format!("{}/send_email", BLOCK_MESH_EMAILS))
            .query(&SendEmail {
                code: env::var("EMAILS_CODE").unwrap_or_default(),
                user_id: user.id,
                email_type: "reset_password".to_string(),
                email_address: user.email,
                nonce: nonce.nonce.expose_secret().clone(),
            })
            .send()
            .await
            .map_err(Error::from)?;
    } else {
        state
            .email_client
            .send_reset_password_email_smtp(&user.email, nonce.nonce.expose_secret())
            .await?;
    }
    commit_txn(transaction).await?;
    Ok(NotificationRedirect::redirect(
        "Email Sent",
        "Please check your email",
        "/login",
    ))
}
