#![allow(clippy::blocks_in_conditions)]
use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use async_trait::async_trait;
use axum_login::tower_sessions::cookie::time::Duration;
use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayer, AuthManagerLayerBuilder, AuthUser, AuthnBackend, UserId,
};
use bcrypt::verify;
use secret::Secret;
use serde::Deserialize;
use sqlx::PgPool;
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
    pub email: String,
    pub nonce: String,
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = SessionUser;
    type Credentials = Credentials;
    type Error = Error;

    #[tracing::instrument(name = "authenticate", skip(creds), err, ret, level = "trace")]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let mut transaction = self.db.begin().await.map_err(Error::from)?;
        let user = get_user_opt_by_email(&mut transaction, &creds.email)
            .await
            .map_err(|e| Error::Auth(e.to_string()))?
            .ok_or_else(|| Error::Auth("User not found".to_string()))?;
        if !verify(creds.password.as_ref(), user.password.as_ref())? {
            return Err(Error::Auth("Invalid password".to_string()));
        }
        transaction.commit().await.map_err(Error::from)?;
        Ok(Option::from(SessionUser {
            id: user.id,
            nonce: creds.nonce,
            email: user.email,
        }))
    }

    #[tracing::instrument(name = "get_user", err, ret, level = "trace")]
    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut transaction = self.db.begin().await.map_err(Error::from)?;
        let user = get_user_opt_by_id(&mut transaction, user_id)
            .await
            .map_err(Error::from)?
            .ok_or_else(|| Error::Auth("User not found".to_string()))?;
        let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
            .await?
            .ok_or_else(|| Error::Auth("Nonce not found".to_string()))?;
        transaction
            .commit()
            .await
            .map_err(|e| Error::Auth(e.to_string()))?;
        Ok(Option::from(SessionUser {
            id: user.id,
            email: user.email,
            nonce: nonce.nonce.as_ref().to_string(),
        }))
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
