use crate::database::spam_email::get_spam_emails::{
    get_from_email_rate_limit, get_spam_emails_cache, update_email_rate_limit,
};
use crate::domain::spam_email::SpamEmail;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use axum::extract::State;
use axum::Json;
use block_mesh_common::constants::{DeviceType, BLOCK_MESH_EMAILS};
use block_mesh_common::interfaces::server_api::{DashboardRequest, SendEmail};
use block_mesh_common::reqwest::http_client;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use chrono::{Duration, Utc};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::Serialize;
use std::env;
use std::sync::Arc;

#[derive(Serialize)]
pub struct ResendConfirmEmailResponse {
    pub message: String,
}

#[tracing::instrument(name = "resend_confirm_email_api", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<DashboardRequest>,
) -> Result<Json<ResendConfirmEmailResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();

    // Validate API token
    let mut transaction = create_txn(&state.pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &email)
        .await?
        .ok_or(Error::UserNotFound)?;

    if user.token.as_ref() != &body.api_token {
        commit_txn(transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    commit_txn(transaction).await?;

    // Check spam domains
    let spam_emails = get_spam_emails_cache().await;
    let email_domain = match email.split('@').next_back() {
        Some(d) => d.to_string(),
        None => {
            return Err(Error::BadRequest("Invalid email domain".to_string()));
        }
    };

    if SpamEmail::check_domains(&email_domain, spam_emails).is_err() {
        return Err(Error::BadRequest("Invalid email domain".to_string()));
    }

    // Rate limiting
    if get_from_email_rate_limit(&user.email).await.is_some()
        || get_from_email_rate_limit(&email).await.is_some()
    {
        return Err(Error::NotAllowedRateLimit);
    }

    let date = Utc::now() + Duration::milliseconds(60_000);
    update_email_rate_limit(&user.email, Some(date)).await;
    update_email_rate_limit(&email, Some(date)).await;

    // Send email
    let email_mode = get_envar("EMAIL_MODE").await;
    if email_mode == "AWS" {
        state
            .email_client
            .send_confirmation_email_aws(&user.email, user.nonce.expose_secret())
            .await?;
    } else if email_mode == "SERVICE" {
        let client = http_client(DeviceType::AppServer);
        client
            .get(format!("{}/send_email", BLOCK_MESH_EMAILS))
            .query(&SendEmail {
                code: env::var("EMAILS_CODE").unwrap_or_default(),
                user_id: user.user_id,
                email_type: "confirm_email".to_string(),
                email_address: user.email,
                nonce: user.nonce.expose_secret().clone(),
            })
            .send()
            .await
            .map_err(Error::from)?;
    } else {
        state
            .email_client
            .send_confirmation_email_smtp(&user.email, user.nonce.expose_secret())
            .await?;
    }

    Ok(Json(ResendConfirmEmailResponse {
        message: "Verification email sent".to_string(),
    }))
}
