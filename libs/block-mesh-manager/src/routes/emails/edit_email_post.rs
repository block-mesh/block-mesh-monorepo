use crate::database::spam_email::get_spam_emails::{
    get_from_email_rate_limit, get_spam_emails_cache, update_email_rate_limit,
};
use crate::domain::spam_email::SpamEmail;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::constants::{DeviceType, BLOCK_MESH_EMAILS};
use block_mesh_common::interfaces::server_api::{EditEmailForm, SendEmail};
use block_mesh_common::reqwest::http_client;
use chrono::{Duration, Utc};
use http::{HeaderMap, StatusCode};
use std::env;
use std::sync::Arc;
use validator::validate_email;

#[tracing::instrument(name = "edit_email_post", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<EditEmailForm>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let email = form.new_email.clone().to_ascii_lowercase();
    if !user.email.starts_with("wallet_") || !user.email.ends_with("@blockmesh.xyz") {
        return Err(Error::Anyhow(anyhow!("Can't change email")));
    }
    let spam_emails = get_spam_emails_cache().await;
    let email_domain = match email.split('@').last() {
        Some(d) => d.to_string(),
        None => {
            return Err(Error::Anyhow(anyhow!(
                "Please check if email you inserted is correct"
            )));
        }
    };

    if SpamEmail::check_domains(&email_domain, spam_emails).is_err() {
        return Err(Error::Anyhow(anyhow!(
            "Please check if email you inserted is correct"
        )));
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

    if get_from_email_rate_limit(&user.email).await.is_some()
        || get_from_email_rate_limit(&email).await.is_some()
        || get_from_email_rate_limit(&header_ip).await.is_some()
    {
        return Err(Error::NotAllowedRateLimit);
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    update_email_rate_limit(&user.email, Some(date)).await;
    update_email_rate_limit(&email, Some(date)).await;
    update_email_rate_limit(&header_ip, Some(date)).await;

    if !validate_email(email.clone()) {
        return Err(Error::Anyhow(anyhow!("Invalid Email")));
    }
    let email_mode = get_envar("EMAIL_MODE").await;
    if email_mode == "AWS" {
        state
            .email_client
            .send_confirmation_email_aws(&email, &user.nonce)
            .await?;
    } else if email_mode == "SERVICE" {
        let client = http_client(DeviceType::AppServer);
        client
            .get(format!("{}/send_email", BLOCK_MESH_EMAILS))
            .query(&SendEmail {
                code: env::var("EMAILS_CODE").unwrap_or_default(),
                user_id: user.id,
                email_type: "confirm_email".to_string(),
                email_address: email,
                nonce: user.nonce,
            })
            .send()
            .await
            .map_err(Error::from)?;
    } else {
        state
            .email_client
            .send_confirmation_email_smtp(&user.email, &user.nonce)
            .await?;
    }
    Ok((StatusCode::OK, "OK").into_response())
}
