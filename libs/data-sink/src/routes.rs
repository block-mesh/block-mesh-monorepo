use crate::data_sink::{now_backup, DataSinkClickHouse};
use crate::errors::Error;
use crate::DataSinkAppState;
use anyhow::anyhow;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use block_mesh_common::interfaces::server_api::DigestDataRequest;
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::user::UserAndApiToken;
use chrono::{Duration, Utc};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use solana_sdk::signature::{Signature, Signer};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;
use validator::validate_email;

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(State(state): State<DataSinkAppState>) -> Result<impl IntoResponse, Error> {
    let clickhouse = &state.clickhouse_client;
    let result: Option<String> = clickhouse
        .query(r#"SELECT current_database()"#)
        .fetch_optional()
        .await
        .map_err(|e| anyhow!(e.to_string()))?;
    tracing::info!("result {:#?}", result);
    match result {
        Some(v) => {
            if v == "default" {
                Ok((StatusCode::OK, "OK"))
            } else {
                Err(Error::from(anyhow!("Wrong DB")))
            }
        }
        None => Err(Error::from(anyhow!("No DB"))),
    }
}

#[tracing::instrument(name = "follower_health", skip_all)]
pub async fn follower_health(
    State(state): State<DataSinkAppState>,
) -> Result<impl IntoResponse, Error> {
    let follower_db_pool = &state.follower_db_pool;
    let mut transaction = create_txn(follower_db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

type CacheType = OnceCell<Arc<RwLock<HashSet<(String, String)>>>>;
static CACHE: CacheType = OnceCell::const_new();
static USER_CACHE: OnceCell<Arc<RwLock<HashMapWithExpiry<String, UserAndApiToken>>>> =
    OnceCell::const_new();

pub async fn digest_data(
    State(state): State<DataSinkAppState>,
    Json(body): Json<DigestDataRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = body.email.to_lowercase();
    if !validate_email(&email) {
        return Err(Error::from(anyhow!("BadEmail")));
    }
    if state.enforce_signature {
        tracing::info!("enforcing sign");
        let pubkey = body.pubkey.ok_or(Error::from(anyhow!("Missing pubkey")))?;
        let signature = body
            .signature
            .ok_or(Error::from(anyhow!("Missing signature")))?;
        let msg = body.msg.ok_or(Error::from(anyhow!("Missing msg")))?;

        if state.ext_keypair.pubkey().to_string() != pubkey {
            return Err(Error::from(anyhow!("Mismatch on keys")));
        }
        let sig =
            Signature::from_str(&signature).map_err(|e| Error::from(anyhow!(e.to_string())))?;
        if !sig.verify(&state.ext_keypair.pubkey().to_bytes(), msg.as_bytes()) {
            return Err(Error::from(anyhow!("Failed to verify signature")));
        }
    }
    let user_cache = USER_CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMapWithExpiry::new())) })
        .await;
    let (user, to_save) = match user_cache.read().await.get(&email).await {
        Some(user) => (user, false),
        None => {
            let follower_db_pool = &state.follower_db_pool;
            let mut transaction = create_txn(follower_db_pool).await?;
            let user = get_user_and_api_token_by_email(&mut transaction, &email)
                .await?
                .ok_or_else(|| anyhow!("UserNotFound"))?;
            if user.token.as_ref() != &body.api_token {
                commit_txn(transaction).await?;
                return Err(Error::from(anyhow!("ApiTokenNotFound")));
            }
            commit_txn(transaction).await?;
            (user, true)
        }
    };
    if to_save {
        let date = Utc::now() + Duration::milliseconds(600_000);
        user_cache
            .write()
            .await
            .insert(email.clone(), user.clone(), Some(date))
            .await;
    }
    if state.use_clickhouse {
        let cache = CACHE
            .get_or_init(|| async { Arc::new(RwLock::new(HashSet::new())) })
            .await;
        let key = (body.data.origin.clone(), body.data.id.clone());
        if cache.read().await.get(&key).is_some() {
            return Ok((StatusCode::ALREADY_REPORTED, "Already reported"));
        }
        let now = Utc::now().timestamp_nanos_opt().unwrap_or(now_backup());
        let row = DataSinkClickHouse {
            id: Uuid::new_v4(),
            user_id: user.user_id,
            raw: body.data.raw,
            origin: body.data.origin,
            origin_id: body.data.id,
            user_name: body.data.user_name,
            link: body.data.link,
            created_at: now as u64,
            updated_at: now as u64,
            reply: body.data.reply.unwrap_or_default(),
            retweet: body.data.retweet.unwrap_or_default(),
            like: body.data.like.unwrap_or_default(),
            tweet: body.data.tweet.unwrap_or_default(),
        };
        let _ = state.tx.send_async(row).await;
        cache.write().await.insert(key);
    }
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
pub fn get_router(state: DataSinkAppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/follower_health", get(follower_health))
        .route("/version", get(version))
        .route("/digest_data", post(digest_data))
        .with_state(state)
}
