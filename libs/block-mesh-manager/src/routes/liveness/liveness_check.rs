use crate::errors::error::Error;
use crate::startup::application::AppState;
use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::extract::{Query, State};
use axum::Json;
use block_mesh_common::feature_flag_client::{get_flag_value_from_map, FlagValue};
use block_mesh_common::interfaces::db_messages::{
    AggregateAddToMessage, DBMessage, DBMessageTypes,
};
use block_mesh_common::interfaces::server_api::{LivenessRequest, LivenessResponse};
use block_mesh_common::rand::init_rand;
use block_mesh_common::solana::get_keypair;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use chrono::Utc;
use database_utils::utils::instrument_wrapper::create_txn;
use solana_sdk::signature::{Signature, Signer};
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[tracing::instrument(name = "liveness_check", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LivenessRequest>,
) -> Result<impl IntoResponse, Error> {
    let timestamp_buffer = env::var("TIMESTAMP_BUFFER")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut follower_transaction, &query.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let pubkey = query.pubkey;
    let signature = query.signature;
    let msg = query.msg;
    let timestamp = query.timestamp;
    let uuid = query.uuid;
    let split: Vec<String> = msg.split("___").map(String::from).collect();
    let now = get_flag_value_from_map(
        state.flags.clone(),
        "block_time",
        FlagValue::Number(Utc::now().timestamp() as f64),
    )
    .await;
    let now: i64 = <FlagValue as TryInto<f64>>::try_into(now.to_owned()).unwrap_or_default() as i64;
    if now > timestamp + timestamp_buffer {
        return Err(Error::from(anyhow!("Timestamp too old")));
    }
    let timestamp_split = split.first().unwrap_or(&"".to_string()).clone();
    if timestamp_split != timestamp.to_string() {
        return Err(Error::from(anyhow!("Timestamp mismatch")));
    }
    let uuid_split = split.get(1).unwrap_or(&"".to_string()).clone();
    if uuid_split != uuid.to_string() {
        return Err(Error::from(anyhow!("uuid mismatch")));
    }
    let keypair = get_keypair()?;
    if keypair.pubkey().to_string() != pubkey {
        return Err(Error::from(anyhow!("Mismatch on keys")));
    }
    let sig = Signature::from_str(&signature).map_err(|e| Error::from(anyhow!(e.to_string())))?;

    if !sig.verify(&keypair.pubkey().to_bytes(), msg.as_bytes()) {
        return Err(Error::from(anyhow!("Failed to verify signature")));
    }
    let _ = notify_worker(
        &state.channel_pool,
        &[DBMessage::AggregateAddToMessage(AggregateAddToMessage {
            msg_type: DBMessageTypes::AggregateAddToMessage,
            user_id: user.user_id,
            value: serde_json::Value::from(1),
            name: AggregateName::InteractiveExt.to_string(),
        })],
    )
    .await;
    let now = Utc::now().timestamp();
    let factor = init_rand(60 * 2, 60 * 5) as i64;
    let response = LivenessResponse {
        timestamp: now + 60 * factor,
    };
    Ok(Json(response).into_response())
}
