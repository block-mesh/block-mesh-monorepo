use crate::error::Error;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, CheckTokenResponseEnum, CheckTokenResponseMap, GetTokenResponse,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use dashmap::try_result::TryResult::Present;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "check_token", skip_all)]
pub async fn check_token(
    Extension(enable_caching): Extension<bool>,
    Extension(pool): Extension<PgPool>,
    Extension(check_token_map): Extension<CheckTokenResponseMap>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.api_token);
    if enable_caching {
        if let Present(entry) = check_token_map.try_get(&key) {
            return match entry.value() {
                CheckTokenResponseEnum::ApiTokenMismatch => {
                    Ok((StatusCode::NO_CONTENT, "Api Token Mismatch").into_response())
                }
                CheckTokenResponseEnum::UserNotFound => {
                    Ok((StatusCode::NO_CONTENT, "User Not Found").into_response())
                }
                CheckTokenResponseEnum::ApiTokenNotFound => {
                    Ok((StatusCode::NO_CONTENT, "Api Token Not Found").into_response())
                }
                CheckTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
            };
        }
    }
    let mut transaction = create_txn(&pool).await?;
    let user = match get_user_and_api_token_by_email(&mut transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                commit_txn(transaction).await?;
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                return Ok((StatusCode::NO_CONTENT, "User Not Found").into_response());
            }
        },
        Err(_) => {
            commit_txn(transaction).await?;
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            return Ok((StatusCode::NO_CONTENT, "User Not Found").into_response());
        }
    };
    if *user.token.as_ref() != body.api_token {
        commit_txn(transaction).await?;
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
        return Ok((StatusCode::NO_CONTENT, "Api Token Mismatch").into_response());
    }
    let response = GetTokenResponse {
        api_token: Some(*user.token.as_ref()),
        message: None,
    };
    commit_txn(transaction).await?;
    check_token_map.insert(
        key,
        CheckTokenResponseEnum::GetTokenResponse(response.clone()),
    );
    Ok(Json(response))
}
