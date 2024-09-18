use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id_pool;
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
use redis::AsyncCommands;
use secret::Secret;
use sqlx::PgPool;
use std::sync::Arc;

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
    if let Ok(token) = c.get(&key).await {
        return Ok(Json(GetTokenResponse {
            api_token: Some(token),
            message: None,
        }));
    }

    let mut transaction = pool.begin().await?;
    let email = body.email.clone().to_ascii_lowercase();
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id_pool(&pool, &user.id)
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
    transaction.commit().await?;

    c.set_ex(
        &key,
        api_token.token.expose_secret().to_string(),
        Backend::get_expire() as u64,
    )
    .await?;

    Ok(Json(GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    }))
}
