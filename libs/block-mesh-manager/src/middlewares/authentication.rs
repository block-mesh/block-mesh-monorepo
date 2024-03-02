/*
use async_trait::async_trait;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId,
};
use serde::Deserialize;
use sqlx::PgPool;
use std::str::FromStr;
use tower_sessions_sqlx_store::PostgresStore;
use uuid::Uuid;

pub type AuthSession = axum_login::AuthSession<Backend>;

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub wallet_address: String,
    pub signed_message: String,
    pub nonce: String,
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: PgPool,
}

impl Backend {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SessionUser {
    pub id: Uuid,
    pub wallet_address: String,
    pub nonce: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let mut transaction = self.db.begin().await.map_err(Error::from)?;
        let user = get_user_opt_by_wallet(&mut transaction, &creds.wallet_address)
            .await
            .map_err(|e| Error::AuthError(e.to_string()))?;
        match user {
            None => {
                return Ok(None);
            }
            Some(user) => {
                let public_key = Pubkey::from_str(&creds.wallet_address).map_err(Error::from)?;
                validate_signature(creds.nonce.as_ref(), &creds.signed_message, &public_key)
                    .map_err(|e| Error::AuthError(e.to_string()))?;
                transaction.commit().await.map_err(Error::from)?;
                Ok(Option::from(SessionUser {
                    id: user.id,
                    wallet_address: user.wallet_address,
                    nonce: creds.nonce,
                }))
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut transaction = self.db.begin().await.map_err(Error::from)?;
        let user = get_user_opt_by_id(&mut transaction, user_id)
            .await
            .map_err(Error::from)?;
        match user {
            None => {
                return Ok(None);
            }
            Some(user) => {
                let nonce = get_nonce(&mut transaction, &user.wallet_address)
                    .await
                    .map_err(|e| Error::AuthError(e.to_string()))?;
                transaction
                    .commit()
                    .await
                    .map_err(|e| Error::AuthError(e.to_string()))?;
                Ok(Option::from(SessionUser {
                    id: user.id,
                    wallet_address: user.wallet_address,
                    nonce: nonce.nonce.as_ref().to_string(),
                }))
            }
        }
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

pub async fn authentication_layer(pool: &PgPool) -> AuthManagerLayer<Backend, PostgresStore> {
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let _deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let backend = Backend::new(pool.clone());
    AuthManagerLayerBuilder::new(backend, session_layer).build()
}
 */
