use crate::errors::Error;
use crate::state::AppState;
use crate::websocket::handle_socket::handle_socket;
use anyhow::{anyhow, Context};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use block_mesh_manager_database_domain::domain::find_token::find_token;
use block_mesh_manager_database_domain::domain::get_user_opt_by_email::get_user_opt_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::HeaderMap;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{span, Level};
use uuid::Uuid;

#[tracing::instrument(name = "ws_handler", skip_all)]
pub async fn ws_handler(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
    let email = query
        .get("email")
        .ok_or(anyhow!("Missing email".to_string()))?
        .clone();
    let api_token = query
        .get("api_token")
        .ok_or(anyhow!("Missing token".to_string()))?;
    let api_token = Uuid::from_str(api_token).context("Cannot deserialize UUID")?;
    let pool = state.pool.clone();
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_opt_by_email(&mut *transaction, &email)
        .await?
        .ok_or(anyhow!(String::from("User email is not present in DB")))?;
    let api_token = find_token(&mut transaction, &api_token)
        .await?
        .ok_or(anyhow!("Api Token Not Found"))?;
    commit_txn(transaction).await?;
    if user.id != api_token.user_id {
        return Err(Error::from(anyhow!("User Not Found")));
    }
    let app_env = env::var("APP_ENVIRONMENT").unwrap_or("production".to_string());
    let header_ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    }
    .to_string();

    Ok(ws.on_upgrade(move |socket| handle_socket(socket, header_ip, state, user.id)))
}
