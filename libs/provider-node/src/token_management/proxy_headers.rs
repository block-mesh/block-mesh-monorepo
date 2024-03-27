use crate::app_state::AppState;
use anyhow::anyhow;
use block_mesh_solana_client::manager::EndpointNodeToProviderNodeHeader;
use std::sync::Arc;

pub async fn process_proxy_headers(
    _app_state: Arc<AppState>,
    req: &mut axum::http::Request<hyper::body::Incoming>,
) -> anyhow::Result<EndpointNodeToProviderNodeHeader> {
    let proxy_authorization = req.headers().get("Proxy-Authorization");
    let auth_header: EndpointNodeToProviderNodeHeader = match proxy_authorization {
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
    Ok(auth_header)
}
