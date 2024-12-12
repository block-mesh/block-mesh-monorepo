use crate::errors::Error;
use crate::state::{WsAppState, WsCredsCache};
use crate::websocket::handle_socket_light::handle_socket_light;
use anyhow::{anyhow, Context};
use axum::extract::{Query, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use block_mesh_common::interfaces::db_messages::{DBMessage, DBMessageTypes, UsersIpMessage};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::user::UserAndApiToken;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::{HeaderMap, StatusCode};
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_user_from_db(
    follower_pool: &PgPool,
    email: &str,
) -> anyhow::Result<Option<UserAndApiToken>> {
    let follower_pool = &follower_pool;
    let mut transaction = create_txn(follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &email).await?;
    commit_txn(transaction).await?;
    Ok(user)
}

#[tracing::instrument(name = "ws_handler", skip_all)]
pub async fn ws_handler(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
    State(state): State<Arc<WsAppState>>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, Error> {
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
    let email = query
        .get("email")
        .ok_or(anyhow!("Missing email".to_string()))?
        .clone();
    let api_token = query
        .get("api_token")
        .ok_or(anyhow!("Missing token".to_string()))?;
    let api_token = Uuid::from_str(api_token).context("Cannot deserialize UUID")?;
    if state.emails.read().await.contains(&email) {
        return Ok((StatusCode::ALREADY_REPORTED, "Already connected").into_response());
    }
    let creds_key = (email.clone(), api_token);
    let mut creds_cache = state.creds_cache.write().await;
    let cached_value = creds_cache.get(&creds_key);
    let user: UserAndApiToken = match cached_value {
        None => match get_user_from_db(&state.follower_pool, &email).await {
            Ok(opt_user) => {
                let user = match opt_user {
                    Some(user) => user,
                    None => {
                        creds_cache.insert(creds_key.clone(), WsCredsCache::UserNotFound);
                        return Ok((StatusCode::NO_CONTENT, "User email is not present in DB")
                            .into_response());
                    }
                };
                if user.token.as_ref() != &api_token {
                    creds_cache.insert(creds_key, WsCredsCache::TokenMismatch);
                    return Ok((StatusCode::NO_CONTENT, "Api Token Mismatch").into_response());
                }
                creds_cache.insert(creds_key, WsCredsCache::Found(user.clone()));
                user
            }
            Err(_) => {
                return Ok((StatusCode::NO_CONTENT, "DB Error").into_response());
            }
        },
        Some(v) => match v {
            WsCredsCache::UserNotFound => {
                return Ok(
                    (StatusCode::NO_CONTENT, "User email is not present in DB").into_response()
                );
            }
            WsCredsCache::TokenMismatch => {
                return Ok((StatusCode::NO_CONTENT, "Api Token Mismatch").into_response());
            }
            WsCredsCache::Found(u) => u.clone(),
        },
    };
    drop(creds_cache);
    let tx_c = state.tx.clone();
    let _ = tx_c
        .send_async(DBMessage::UsersIpMessage(UsersIpMessage {
            msg_type: DBMessageTypes::UsersIpMessage,
            id: user.user_id,
            ip: header_ip.clone(),
        }))
        .await;

    Ok(ws.on_upgrade(move |socket| {
        handle_socket_light(email, socket, header_ip, state, user.user_id)
    }))
}
