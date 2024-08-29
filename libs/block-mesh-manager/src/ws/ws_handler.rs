use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::ws::handle_socket::handle_socket;
use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use redis::AsyncCommands;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    tracing::info!("query => {:#?}", query);
    let email = query
        .get("email")
        .ok_or(Error::Auth("Missing email".to_string()))?
        .clone();
    let api_token = query
        .get("api_token")
        .ok_or(Error::Auth("Missing token".to_string()))?;
    let mut c = state.redis.clone();

    // Checks for key that does not exist after logging in?
    // let _: String = c
    //     .get(format!(
    //         "{}-{}",
    //         email.clone().to_ascii_lowercase(),
    //         api_token.to_string()
    //     ))
    //     .await
    //     .map_err(|_| Error::Auth("Can't find token".to_string()))?;

    tracing::info!("ws_handle => connected {:#?}", query);
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, addr, state, email)))
}
