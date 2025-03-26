use crate::errors::Error;
use crate::state::{WsAppState, WsCredsCache};
use crate::websocket::handle_socket_light::handle_socket_light;
use anyhow::{anyhow, Context};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum_tws::WebSocketUpgrade;
use block_mesh_common::interfaces::db_messages::{
    AggregateAddToMessage, DBMessage, DBMessageTypes, UsersIpMessage,
};
use block_mesh_common::solana::get_keypair;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::user::UserAndApiToken;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use flume::Sender;
use http::{HeaderMap, StatusCode};
use serde_json::Value;
use solana_sdk::signature::{Signature, Signer};
use sqlx::types::chrono::Utc;
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
    let user = get_user_and_api_token_by_email(&mut transaction, email).await?;
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
    let timestamp_buffer = env::var("TIMESTAMP_BUFFER")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
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
    let enforce_keypair = env::var("ENFORCE_KEYPAIR")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let now = Utc::now().timestamp();
    if enforce_keypair {
        let signature = query
            .get("signature")
            .ok_or(anyhow!("Missing signature".to_string()))?
            .clone();
        let pubkey = query
            .get("pubkey")
            .ok_or(anyhow!("Missing pubkey".to_string()))?
            .clone();
        let uuid = query
            .get("uuid")
            .ok_or(anyhow!("Missing uuid".to_string()))?
            .clone();
        let msg = query
            .get("msg")
            .ok_or(anyhow!("Missing msg".to_string()))?
            .clone();
        let timestamp = query
            .get("timestamp")
            .ok_or(anyhow!("Missing timestamp".to_string()))?
            .clone()
            .parse()
            .unwrap_or(0i64);
        if now > timestamp + timestamp_buffer {
            return Err(Error::from(anyhow!("Timestamp too old")));
        }
        let split: Vec<String> = msg.split("___").map(String::from).collect();
        let timestamp_split = split.first().unwrap_or(&"".to_string()).clone();
        if timestamp_split != timestamp.to_string() {
            return Err(Error::from(anyhow!("Timestamp mismatch")));
        }
        let uuid_split = split.get(1).unwrap_or(&"".to_string()).clone();
        if uuid_split != uuid {
            return Err(Error::from(anyhow!("uuid mismatch")));
        }
        let keypair = get_keypair()?;
        if keypair.pubkey().to_string() != pubkey {
            return Err(Error::from(anyhow!("Mismatch on keys")));
        }
        let sig =
            Signature::from_str(&signature).map_err(|e| Error::from(anyhow!(e.to_string())))?;
        if !sig.verify(&keypair.pubkey().to_bytes(), msg.as_bytes()) {
            return Err(Error::from(anyhow!("Failed to verify signature")));
        }
    }

    let email = query
        .get("email")
        .ok_or(anyhow!("Missing email".to_string()))?
        .clone()
        .to_lowercase();
    let api_token = query
        .get("api_token")
        .ok_or(anyhow!("Missing token".to_string()))?;
    let api_token = Uuid::from_str(api_token).context("Cannot deserialize UUID")?;
    if state.emails.read().await.contains(&email) {
        return Ok((StatusCode::ALREADY_REPORTED, "Already connected").into_response());
    }

    let user_in_redis = state.check_email_redis(&email).await?;
    if user_in_redis > 0 {
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
    if let Some(s) = query.get("wootz") {
        if let Ok(wootz) = serde_json::from_str::<Value>(s) {
            let _ = process_woots(wootz, tx_c, &user.user_id).await;
        }
    }
    Ok(ws.on_upgrade(move |socket| {
        handle_socket_light(email, socket, header_ip, state, user.user_id)
    }))
}

pub async fn process_woots(
    wootz: Value,
    tx_c: Sender<DBMessage>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    if wootz.is_object() {
        let wootz = wootz.as_object().unwrap();
        let is_wootz_app = wootz
            .get("isWootzapp")
            .ok_or(anyhow!("Missing isWootzapp"))?
            .as_str()
            .unwrap_or_default();
        let name = wootz
            .get("name")
            .ok_or(anyhow!("name is missing"))?
            .as_str()
            .unwrap_or_default();
        let vendor = wootz
            .get("vendor")
            .ok_or(anyhow!("vendor is missing"))?
            .as_str()
            .unwrap_or_default();
        if is_wootz_app != "true" {
            return Ok(());
        }
        if !name.to_lowercase().contains("wootz") {
            return Ok(());
        }
        if !vendor.to_lowercase().contains("wootz") {
            return Ok(());
        }
        let _ = tx_c
            .send_async(DBMessage::AggregateAddToMessage(AggregateAddToMessage {
                user_id: *user_id,
                name: AggregateName::Wootz.to_string(),
                value: serde_json::Value::from(1),
                msg_type: DBMessageTypes::UsersIpMessage,
            }))
            .await;
    }
    Ok(())
}
