use crate::app_state::AppState;
use crate::proxy_server::tunnel::tunnel;
use axum::body::Body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use block_mesh_solana_client::manager::FullRouteHeader;
use std::sync::Arc;

#[tracing::instrument(name = "proxy", skip(app_state), ret, err)]
pub async fn proxy(app_state: Arc<AppState>, req: Request) -> Result<Response, hyper::Error> {
    tracing::trace!(?req);
    tracing::info!("proxy headers {:?}", req.headers());

    let proxy_authorization = req.headers().get("Proxy-Authorization");

    let solana_manager_auth: FullRouteHeader = match proxy_authorization {
        None => {
            let msg = "proxy authorization header not found";
            tracing::error!(msg);
            return Ok((StatusCode::BAD_REQUEST, msg).into_response());
        }
        Some(proxy_authorization) => {
            match serde_json::from_str(proxy_authorization.to_str().unwrap()) {
                Ok(solana_manager_auth) => solana_manager_auth,
                Err(e) => {
                    let msg = format!("failed to parse proxy authorization header: {}", e);
                    tracing::error!(msg);
                    return Ok((StatusCode::BAD_REQUEST, msg).into_response());
                }
            }
        }
    };

    let token_manager = app_state.token_manager.read().await;
    let token_details = token_manager.get(&solana_manager_auth.api_token);
    let api_token = match token_details {
        None => {
            tracing::warn!("token not found");
            return Ok((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
        }
        Some(token_details) => {
            if !token_details.is_valid(&solana_manager_auth) {
                tracing::warn!("token is not valid");
                return Ok((StatusCode::UNAUTHORIZED, "Unauthorized").into_response());
            } else {
                token_details.api_token
            }
        }
    };

    let app_state = app_state.clone();
    if let Some(host_addr) = req.uri().authority().map(|auth| auth.to_string()) {
        tokio::task::spawn(async move {
            match hyper::upgrade::on(req).await {
                Ok(upgraded) => {
                    if let Err(e) = tunnel(app_state, upgraded, host_addr, api_token).await {
                        tracing::warn!("server io error: {}", e);
                    };
                }
                Err(e) => tracing::warn!("upgrade error: {}", e),
            }
        });
        Ok(Response::new(Body::empty()))
    } else {
        tracing::warn!("CONNECT host is not socket addr: {:?}", req.uri());
        Ok((
            StatusCode::BAD_REQUEST,
            "CONNECT must be to a socket address",
        )
            .into_response())
    }
}
