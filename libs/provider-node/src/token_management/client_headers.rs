use crate::app_state::AppState;
use anyhow::anyhow;
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::FullRouteHeader;
use std::sync::Arc;
use uuid::Uuid;

pub async fn process_client_headers(
    app_state: Arc<AppState>,
    req: &mut axum::http::Request<hyper::body::Incoming>,
) -> anyhow::Result<FullRouteHeader> {
    let proxy_authorization = req.headers().get("Proxy-Authorization");

    let nonce = Uuid::new_v4().to_string();
    let _signed_message =
        sign_message(&nonce, &app_state.solana_manager.read().await.get_keypair()).unwrap();

    let solana_manager_auth: FullRouteHeader = match proxy_authorization {
        None => {
            let msg = "proxy authorization header not found";
            tracing::error!(msg);
            return Err(anyhow!(msg));
        }
        Some(proxy_authorization) => {
            match serde_json::from_str(proxy_authorization.to_str().unwrap()) {
                Ok(solana_manager_auth) => solana_manager_auth,
                Err(e) => {
                    let msg = format!("failed to parse proxy authorization header: {}", e);
                    tracing::error!(msg);
                    return Err(anyhow!(msg));
                }
            }
        }
    };

    let token_manager = app_state.token_manager.read().await;
    let _token_details = token_manager.get(&solana_manager_auth.api_token);
    // match token_details {
    //     None => {
    //         let msg = "token not found";
    //         tracing::warn!(msg);
    //         return Err(anyhow!(msg));
    //     }
    //     Some(token_details) => {
    //         if !token_details.is_valid(&solana_manager_auth) {
    //             let msg = "token is not valid";
    //             tracing::warn!(msg);
    //             return Err(anyhow!(msg));
    //         } else {
    //             token_details.api_token
    //         }
    //     }
    // };
    Ok(solana_manager_auth)
}
