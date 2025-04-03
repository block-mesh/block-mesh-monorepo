use crate::errors::Error;
use crate::{get_or_create_id, IdAppState};
use anyhow::{anyhow, Context};
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use block_mesh_common::interfaces::server_api::IdRequest;
use block_mesh_common::solana::get_keypair;
use dash_with_expiry::hash_set_with_expiry::HashSetWithExpiry;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::{Signature, Signer};
use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

#[tracing::instrument(name = "db_health", skip_all)]
pub async fn db_health(State(state): State<IdAppState>) -> Result<impl IntoResponse, Error> {
    let mut transaction = create_txn(&state.db_pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "server_health", skip_all)]
pub async fn server_health() -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::OK, "OK"))
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
struct DataToCache {
    pub email: String,
    pub api_token: String,
    pub fp: String,
    pub fp2: String,
    pub fp3: String,
    pub fp4: String,
    pub ip: String,
}

type CacheType = Arc<RwLock<HashSetWithExpiry<u64>>>;
static CACHE: OnceCell<CacheType> = OnceCell::const_new();

pub async fn id(
    headers: HeaderMap,
    State(state): State<IdAppState>,
    Query(query): Query<IdRequest>,
) -> Result<impl IntoResponse, Error> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashSetWithExpiry::new())) })
        .await;
    let app_env = env::var("APP_ENVIRONMENT").map_err(|_| Error::Anyhow(anyhow!("Missing env")))?;
    let ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    };
    let c_query = query.clone();
    let data = DataToCache {
        email: c_query.email,
        api_token: c_query.api_token,
        fp: c_query.fp,
        fp2: c_query.fp2.unwrap_or_default(),
        fp3: c_query.fp3.unwrap_or_default(),
        fp4: c_query.fp4.unwrap_or_default(),
        ip: ip.to_string(),
    };
    let mut s = DefaultHasher::new();
    data.hash(&mut s);
    let hash = s.finish();
    if cache.read().await.get(&hash).await.is_some() {
        return Ok((StatusCode::OK, "OK").into_response());
    }
    let timestamp_buffer = env::var("TIMESTAMP_BUFFER")
        .unwrap_or("300".to_string())
        .parse()
        .unwrap_or(300);
    let enforce_keypair = env::var("ENFORCE_KEYPAIR")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    if enforce_keypair {
        let now = *state.block_time.read().await;
        let signature = query
            .signature
            .ok_or(anyhow!("Missing signature".to_string()))?
            .clone();
        let pubkey = query
            .pubkey
            .ok_or(anyhow!("Missing pubkey".to_string()))?
            .clone();
        let uuid = query.uuid.ok_or(anyhow!("Missing uuid".to_string()))?;
        let msg = query.msg.ok_or(anyhow!("Missing msg".to_string()))?.clone();
        let timestamp = query
            .timestamp
            .ok_or(anyhow!("Missing timestamp".to_string()))?;
        if timestamp == 0 {
            return Err(Error::from(anyhow!("Timestamp is empty")));
        }
        if now > timestamp + timestamp_buffer {
            return Err(Error::from(anyhow!("Timestamp too old")));
        }
        let split: Vec<String> = msg.split("___").map(String::from).collect();
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
        let sig =
            Signature::from_str(&signature).map_err(|e| Error::from(anyhow!(e.to_string())))?;
        if !sig.verify(&keypair.pubkey().to_bytes(), msg.as_bytes()) {
            return Err(Error::from(anyhow!("Failed to verify signature")));
        }
    }
    let mut transaction = create_txn(&state.db_pool).await?;
    let _ = get_or_create_id(
        &mut transaction,
        &query.email,
        &query.api_token,
        &query.fp,
        &query.fp2.unwrap_or_default(),
        &query.fp3.unwrap_or_default(),
        &query.fp4.unwrap_or_default(),
        ip,
    )
    .await?;
    commit_txn(transaction).await?;
    let c = cache.write().await;
    c.insert(hash, None).await;
    Ok((StatusCode::OK, "OK").into_response())
}
#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}
pub fn get_router(state: IdAppState) -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/version", get(version))
        .route("/id", get(id))
        .with_state(state)
}
