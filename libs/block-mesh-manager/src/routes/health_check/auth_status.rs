use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::AuthStatusResponse;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use dashmap::try_result::TryResult::Present;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::sync::Arc;

#[tracing::instrument(name = "auth_status", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<AuthStatusResponse>, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let wallet_address = if let Present(entry) = state.wallet_addresses.try_get(&user.email) {
        entry.value().clone()
    } else {
        let mut transaction = create_txn(&state.follower_pool).await?;
        let db_user = get_user_opt_by_id(&mut transaction, &user.id)
            .await
            .map_err(Error::from)?;
        commit_txn(transaction).await?;
        match db_user {
            Some(user) => {
                state
                    .wallet_addresses
                    .insert(user.email.clone(), user.wallet_address.clone());
                user.wallet_address.clone()
            }
            None => {
                return Ok(Json(AuthStatusResponse {
                    enable_proof_of_humanity: state.enable_proof_of_humanity,
                    email: None,
                    status_code: 404,
                    logged_in: false,
                    wallet_address: None,
                }))
            }
        }
    };
    Ok(Json(AuthStatusResponse {
        enable_proof_of_humanity: state.enable_proof_of_humanity,
        email: Some(user.email.to_ascii_lowercase()),
        status_code: 200,
        logged_in: true,
        wallet_address,
    }))
}
