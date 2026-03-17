use crate::database::user::update_extension_activated::update_extension_activated;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::snag::sync_first_activation;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    ActivateExtensionRequest, ActivateExtensionResponse,
};
use block_mesh_common::solana::get_keypair;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use solana_sdk::signature::{Signature, Signer};
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[tracing::instrument(name = "activate_extension", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Json(body): Json<ActivateExtensionRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = body.email.to_ascii_lowercase();
    let timestamp_buffer = env::var("TIMESTAMP_BUFFER")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let now = Utc::now().timestamp();

    if body.timestamp < now - timestamp_buffer || body.timestamp > now + timestamp_buffer {
        return Err(Error::BadRequest(
            "Timestamp outside allowed window".to_string(),
        ));
    }

    let expected_msg = format!("{}___{}", email, body.timestamp);
    if body.msg != expected_msg {
        return Err(Error::BadRequest("Message mismatch".to_string()));
    }

    let keypair = get_keypair()?;
    let signature = Signature::from_str(&body.signature)
        .map_err(|e| Error::BadRequest(format!("Invalid signature: {e}")))?;
    if !signature.verify(&keypair.pubkey().to_bytes(), body.msg.as_bytes()) {
        return Err(Error::BadRequest("Failed to verify signature".to_string()));
    }

    let mut transaction = create_txn(&pool).await?;
    let user_and_api_token = get_user_and_api_token_by_email(&mut transaction, &email)
        .await?
        .ok_or(Error::UserNotFound)?;

    if *user_and_api_token.token.as_ref() != body.api_token {
        commit_txn(transaction).await?;
        return Err(Error::Unauthorized);
    }

    let activated_now =
        update_extension_activated(&mut transaction, &user_and_api_token.user_id, true).await?;
    commit_txn(transaction).await?;

    if activated_now {
        let client = state.client.clone();
        let snag = state.snag.clone();
        let user_id = user_and_api_token.user_id;
        let user_email = user_and_api_token.email.clone();
        let wallet_address = user_and_api_token.wallet_address.clone();
        tokio::spawn(async move {
            if let Err(error) =
                sync_first_activation(client, snag, user_id, user_email, wallet_address).await
            {
                tracing::warn!("failed to sync first activation to Snag: {error}");
            }
        });
    }

    Ok((
        StatusCode::OK,
        Json(ActivateExtensionResponse {
            status_code: u16::from(StatusCode::OK),
        }),
    ))
}
