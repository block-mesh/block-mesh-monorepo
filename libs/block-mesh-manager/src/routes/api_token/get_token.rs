use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::verify_cache::verify_with_cache;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    GetTokenRequest, GetTokenResponse, GetTokenResponseEnum,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use dashmap::try_result::TryResult::Present;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "get_token", skip(pool, state))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetTokenRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.password.clone());
    let get_token_map = &state.get_token_map;
    if let Present(entry) = get_token_map.try_get(&key) {
        return match entry.value() {
            GetTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone()).into_response()),
            GetTokenResponseEnum::UserNotFound => {
                Ok((StatusCode::OK, "User Not Found").into_response())
            }
            GetTokenResponseEnum::PasswordMismatch => {
                Ok((StatusCode::OK, "Password Mismatch").into_response())
            }
            GetTokenResponseEnum::ApiTokenNotFound => {
                Ok((StatusCode::OK, "Api Token Not Found").into_response())
            }
        };
    }
    let mut transaction = create_txn(&pool).await?;
    let user_and_api_token = match get_user_and_api_token_by_email(&mut transaction, &email).await {
        Ok(user) => match user {
            Some(user_and_api_token) => user_and_api_token,
            None => {
                commit_txn(transaction).await?;
                get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
                return Ok((StatusCode::OK, "User Not Found").into_response());
            }
        },
        Err(_) => {
            commit_txn(transaction).await?;
            get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
            return Ok((StatusCode::OK, "User Not Found").into_response());
        }
    };
    if !verify_with_cache(body.password.as_ref(), user_and_api_token.password.as_ref()).await {
        commit_txn(transaction).await?;
        get_token_map.insert(key, GetTokenResponseEnum::PasswordMismatch);
        return Ok((StatusCode::OK, "Password Mismatch").into_response());
    }
    let response = GetTokenResponse {
        api_token: Some(*user_and_api_token.token.as_ref()),
        message: None,
    };
    commit_txn(transaction).await?;
    get_token_map.insert(
        key,
        GetTokenResponseEnum::GetTokenResponse(response.clone()),
    );
    Ok(Json(response).into_response())
}
