use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status_pool;
use crate::database::user::get_user_by_email::get_user_opt_by_email_pool;
use crate::domain::api_token::ApiTokenStatus;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetTokenResponse};
use redis::AsyncCommands;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "check_token", skip_all, level = "trace", fields(email=body.email))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let key = Backend::authenticate_key_with_api_token(
        &body.email.to_ascii_lowercase(),
        &body.api_token.to_string(),
    );
    let mut c = state.redis.clone();
    if let Ok(token) = c.get(&key).await {
        return Ok(Json(GetTokenResponse {
            api_token: Some(token),
            message: None,
        }));
    }
    let email = body.email.clone().to_ascii_lowercase();
    let user = get_user_opt_by_email_pool(&pool, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let api_token = get_api_token_by_usr_and_status_pool(&pool, &user.id, ApiTokenStatus::Active)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    if *api_token.token.as_ref() != body.api_token {
        return Err(Error::ApiTokenMismatch);
    }

    c.set_ex(
        &key,
        body.api_token.to_string(),
        Backend::get_expire() as u64,
    )
    .await?;
    Ok(Json(GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    }))
}
