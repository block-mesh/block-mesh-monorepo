#![allow(clippy::blocks_in_conditions)]

use crate::errors::error::Error;
use crate::utils::cache_envar::get_envar;
use crate::utils::verify_cache::verify_with_cache;
use anyhow::anyhow;
use async_trait::async_trait;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_user_id::get_user_and_api_token_by_user_id;
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use redis::aio::MultiplexedConnection;
use secret::Secret;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::OnceCell;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: Secret<String>,
    pub nonce: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Backend {
    db: PgPool,
    con: MultiplexedConnection,
}

static CACHE: OnceCell<Arc<HashMapWithExpiry<String, SessionUser>>> = OnceCell::const_new();

impl Backend {
    pub async fn get_expire() -> i64 {
        get_envar("CACHE_EXPIRE").await.parse().unwrap_or(86400)
    }
    pub fn new(db: PgPool, con: MultiplexedConnection) -> Self {
        Self { db, con }
    }

    pub fn authenticate_key_with_password(email: &str, password: &Secret<String>) -> String {
        format!("{}-{}", email, password.expose_secret())
    }
    pub fn authenticate_key_with_api_token(email: &str, api_token: &str) -> String {
        format!("{}-{}", email, api_token)
    }

    pub fn authenticate_key_with_user_id(uuid: &Uuid) -> String {
        uuid.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
    pub nonce: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = Error;

    #[tracing::instrument(name = "authenticate", skip_all)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let key = Backend::authenticate_key_with_password(&creds.email, &creds.password);
        if let Ok(cache_user) = get_user_from_cache(&key).await {
            return Ok(Some(cache_user));
        }
        let pool = self.db.clone();
        let mut transaction = create_txn(&pool).await?;
        let user = match get_user_and_api_token_by_email(&mut transaction, &creds.email).await {
            Ok(u) => u,
            Err(e) => {
                del_from_cache(&key).await;
                return Err(Error::Auth(e.to_string()));
            }
        };
        commit_txn(transaction).await?;
        let user = match user {
            Some(u) => u,
            None => {
                del_from_cache(&key).await;
                return Err(Error::Auth("User not found".to_string()));
            }
        };
        if !verify_with_cache(creds.password.as_ref(), user.password.as_ref()).await {
            return Err(Error::Auth("Invalid password".to_string()));
        }
        let session_user = SessionUser {
            id: user.user_id,
            nonce: creds.nonce,
            email: user.email,
        };
        save_to_cache(&key, &session_user).await;
        let key = Backend::authenticate_key_with_user_id(&user.user_id);
        save_to_cache(&key, &session_user).await;
        Ok(Option::from(session_user))
    }

    #[tracing::instrument(name = "get_user", skip_all)]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let key = Backend::authenticate_key_with_user_id(user_id);
        if let Ok(cache_user) = get_user_from_cache(&user_id.to_string()).await {
            return Ok(Some(cache_user));
        }
        let pool = self.db.clone();
        let mut transaction = create_txn(&pool).await?;
        let user = match get_user_and_api_token_by_user_id(&mut transaction, user_id).await {
            Ok(u) => u,
            Err(e) => {
                del_from_cache(&key).await;
                return Err(Error::Auth(e.to_string()));
            }
        };

        let user = match user {
            Some(u) => u,
            None => {
                del_from_cache(&key).await;
                return Err(Error::Auth("User not found".to_string()));
            }
        };

        let session_user = SessionUser {
            id: user.user_id,
            email: user.email.clone(),
            nonce: user.nonce.as_ref().to_string(),
        };
        save_to_cache(&key, &session_user).await;
        commit_txn(transaction).await?;
        Ok(Option::from(session_user))
    }
}

impl AuthUser for SessionUser {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.nonce.as_bytes() // We use the password hash as the auth
                              // hash--what this means
                              // is when the user changes their password the
                              // auth session becomes invalid.
    }
}

pub async fn authentication_layer(
    pool: &PgPool,
    con: &MultiplexedConnection,
) -> AuthManagerLayer<Backend, PostgresStore> {
    let inactivity_limit = get_envar("INACTIVITY_LIMIT").await;
    let inactivity_limit = inactivity_limit.parse().unwrap_or(900_000);

    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let _deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::milliseconds(
            inactivity_limit,
        )));

    let backend = Backend::new(pool.clone(), con.clone());
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}

#[tracing::instrument(name = "get_user_from_cache", skip_all)]
pub async fn get_user_from_cache(key: &str) -> anyhow::Result<SessionUser> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(HashMapWithExpiry::new(1_000)) })
        .await;
    match cache.get(&key.to_string()).await {
        Some(user) => Ok(user.clone()),
        None => Err(anyhow!("User not found".to_string())),
    }
}

#[tracing::instrument(name = "save_to_cache", skip_all)]
pub async fn save_to_cache(key: &str, session_user: &SessionUser) {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(HashMapWithExpiry::new(1_000)) })
        .await;
    cache
        .insert(key.to_string(), session_user.clone(), None)
        .await;
}

#[tracing::instrument(name = "del_from_cache", skip_all)]
pub async fn del_from_cache(key: &str) {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(HashMapWithExpiry::new(1_000)) })
        .await;
    cache.remove(&key.to_string()).await;
}

#[tracing::instrument(name = "del_from_cache_with_pattern", skip_all, err)]
pub async fn del_from_cache_with_pattern(key: &str) -> anyhow::Result<()> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(HashMapWithExpiry::new(1_000)) })
        .await;
    let keys = cache.keys().await;
    for k in keys {
        if k.starts_with(key) {
            cache.remove(&k.to_string()).await;
        }
    }
    Ok(())
}
