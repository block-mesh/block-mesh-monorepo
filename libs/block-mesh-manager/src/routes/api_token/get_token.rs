use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::domain::api_token::ApiTokenStatus;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use anyhow::anyhow;
use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{GetTokenRequest, GetTokenResponse};
use redis::{AsyncCommands, RedisResult};
use secret::Secret;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "get_token", skip(body, auth, state), fields(email = body.email))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Json(body): Json<GetTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let key = Backend::authenticate_key_with_password(
        &body.email.to_ascii_lowercase(),
        &Secret::from(body.password.clone()),
    );
    let mut c = state.redis.clone();
    let token: RedisResult<String> = c.get(&key).await;
    if let Ok(token) = token {
        if let Ok(token) = Uuid::from_str(&token) {
            return Ok(Json(GetTokenResponse {
                api_token: Some(token),
                message: None,
            }));
        }
    }

    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let email = body.email.clone().to_ascii_lowercase();
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let creds: Credentials = Credentials {
        email,
        password: Secret::from(body.password.clone()),
        nonce: nonce.nonce.as_ref().to_string(),
    };
    let session = auth
        .authenticate(creds)
        .await
        .map_err(|_| Error::Auth(anyhow!("Authentication failed").to_string()))?
        .ok_or(Error::UserNotFound)?;
    auth.login(&session)
        .await
        .map_err(|_| Error::Auth(anyhow!("Login failed").to_string()))?;
    let api_token =
        get_api_token_by_usr_and_status(&mut transaction, &user.id, ApiTokenStatus::Active)
            .await?
            .ok_or(Error::ApiTokenNotFound)?;
    transaction.commit().await.map_err(Error::from)?;

    let _: RedisResult<()> = c
        .set(&key, api_token.token.expose_secret().to_string())
        .await;
    let _: RedisResult<()> = c.expire(&key, Backend::get_expire()).await;

    Ok(Json(GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    }))
}
