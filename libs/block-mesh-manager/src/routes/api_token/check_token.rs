use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::domain::api_token::ApiTokenStatus;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetTokenResponse};
use redis::{AsyncCommands, RedisResult};
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "check_token", skip(body, state), level = "trace", fields(email=body.email))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let mut c = state.redis.clone();
    let token: RedisResult<String> = c
        .get(format!(
            "{}-{}",
            body.email.clone().to_ascii_lowercase(),
            body.api_token.to_string()
        ))
        .await;
    if let Ok(token) = token {
        return Ok(Json(GetTokenResponse {
            api_token: Some(token.parse().unwrap()),
            message: None,
        }));
    }

    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let email = body.email.clone().to_ascii_lowercase();
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let api_token =
        get_api_token_by_usr_and_status(&mut transaction, &user.id, ApiTokenStatus::Active)
            .await?
            .ok_or(Error::ApiTokenNotFound)?;
    if *api_token.token.as_ref() != body.api_token {
        return Err(Error::ApiTokenMismatch);
    }
    transaction.commit().await.map_err(Error::from)?;

    let _: RedisResult<()> = c
        .set(
            format!(
                "{}-{}",
                body.email.clone().to_ascii_lowercase(),
                body.api_token.to_string()
            ),
            api_token.token.to_string(),
        )
        .await;
    let _: RedisResult<()> = c
        .expire(
            format!(
                "{}-{}",
                body.email.clone().to_ascii_lowercase(),
                body.api_token.to_string()
            ),
            60 * 60 * 24,
        )
        .await;

    Ok(Json(GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    }))
}
