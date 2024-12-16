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
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use http::{HeaderMap, StatusCode};
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;
use validator::validate_email;

static RATE_LIMIT_EMAIL: OnceCell<DashSetWithExpiry<String>> = OnceCell::const_new();

#[tracing::instrument(name = "edit_email_post", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<EditEmailForm>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let email = form.new_email.clone().to_ascii_lowercase();

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
    if cache.get(&user.email).is_some()
        || cache.get(&email).is_some()
        || cache.get(&header_ip).is_some()
    {
        return Err(Error::NotAllowedRateLimit);
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    cache.insert(user.email.clone(), Some(date));
    cache.insert(email.clone(), Some(date));
    cache.insert(header_ip, Some(date));

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
