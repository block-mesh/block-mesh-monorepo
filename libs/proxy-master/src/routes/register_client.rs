use crate::app_state::AppState;
use crate::token_management::channels::TokenDetails;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use block_mesh_solana_client::helpers::validate_signature;
use block_mesh_solana_client::manager::FullRouteHeader;
// use blockmesh_program::state::api_token::ApiToken;
use std::sync::Arc;

#[tracing::instrument(name = "register_client", skip(state))]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<FullRouteHeader>,
) -> impl IntoResponse {
    let signature = body.client_signature.signature;
    let nonce = body.client_signature.nonce;
    let pubkey = body.client_signature.pubkey;
    let api_token = body.api_token;
    let validated = validate_signature(&nonce, &signature, &pubkey);
    // let api_token_account: anyhow::Result<ApiToken> = state
    //     .solana_manager
    //     .read()
    //     .await
    //     .get_deserialized_account(&api_token)
    //     .await;
    // match api_token_account {
    //     Ok(api_token_account) => {
    //         if api_token_account.owner != pubkey {
    //             tracing::error!("api token account owner does not match pubkey");
    //             return (StatusCode::UNAUTHORIZED, "Unauthorized");
    //         }
    //     }
    //     Err(e) => {
    //         tracing::error!("failed to get api token account: {}", e);
    //         return (StatusCode::UNAUTHORIZED, "Unauthorized");
    //     }
    // }
    match validated {
        Ok(status) => match status {
            true => {
                let mut token_manager = state.token_manager.write().await;
                token_manager
                    .entry(api_token)
                    .or_insert_with(|| TokenDetails {
                        api_token,
                        bandwidth_allowance: 0,
                        bandwidth_used: 0,
                        nonce,
                        signature,
                        pubkey,
                    });
                // tracing::info!("Registering client {:?}", body);
                (StatusCode::OK, "OK")
            }
            false => {
                tracing::warn!("Failed to validate signature");
                (StatusCode::UNAUTHORIZED, "Unauthorized")
            }
        },
        Err(e) => {
            tracing::warn!("failed to validate signature: {}", e);
            (StatusCode::UNAUTHORIZED, "Unauthorized")
        }
    }
}
