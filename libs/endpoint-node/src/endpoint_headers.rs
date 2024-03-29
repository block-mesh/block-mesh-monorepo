use anyhow::anyhow;
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{FullRouteHeader, SolanaManager};
use hyper::http::HeaderValue;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "process_endpoint_headers", skip(solana_manager), ret, err)]
pub async fn process_endpoint_headers(
    solana_manager: Arc<SolanaManager>,
    req: &mut axum::http::Request<hyper::body::Incoming>,
) -> anyhow::Result<FullRouteHeader> {
    let proxy_authorization = req.headers().get("Proxy-Authorization");
    let mut solana_manager_auth: FullRouteHeader = match proxy_authorization {
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
    let nonce = Uuid::new_v4().to_string();
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair()).unwrap();
    let pubkey = solana_manager.get_pubkey();
    solana_manager_auth.add_endpoint_node_signature(
        nonce,
        signed_message,
        pubkey,
        "endpoint-node".to_string(),
    );
    let json = serde_json::to_string(&solana_manager_auth)?;
    let proxy_authorization = HeaderValue::from_str(&json)?;
    req.headers_mut()
        .insert("Proxy-Authorization", proxy_authorization);

    tracing::info!("process_endpoint_headers req = {:?}", req);
    Ok(solana_manager_auth)
}
