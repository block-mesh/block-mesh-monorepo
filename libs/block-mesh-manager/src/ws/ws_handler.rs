use crate::database::api_token::find_token::find_token;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use crate::utils::instrument_wrapper::{commit_txn, create_txn};
use crate::ws::handle_socket::handle_socket;
use anyhow::Context;
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use http::HeaderMap;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{span, Level};
use uuid::Uuid;

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[tracing::instrument(name = "ws_handler", skip_all)]
pub async fn ws_handler(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    let email = query
        .get("email")
        .ok_or(Error::Auth("Missing email".to_string()))?
        .clone();
    let api_token = query
        .get("api_token")
        .ok_or(Error::Auth("Missing token".to_string()))?;
    let api_token = Uuid::from_str(api_token).context("Cannot deserialize UUID")?;
    let pool = state.pool.clone();
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or(Error::Auth(String::from("User email is not present in DB")))?;
    let api_token = find_token(&mut transaction, &api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    commit_txn(transaction).await?;
    if user.id != api_token.user_id {
        return Err(Error::UserNotFound);
    }
    let app_environment = get_envar("APP_ENVIRONMENT").await;
    let ip = if app_environment != "local" {
        let span = span!(Level::TRACE, "headers").entered();
        let value = headers
            .get("cf-connecting-ip")
            .ok_or(Error::Auth("Missing cf-connecting-ip".to_string()))?
            .to_str()
            .unwrap_or_default()
            .to_string();
        span.exit();
        value
    } else {
        "127.0.0.1".to_string()
    };
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, ip, state, user.id)))
}
