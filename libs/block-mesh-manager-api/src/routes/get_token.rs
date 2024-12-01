use crate::error::Error;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use bcrypt::verify;
use block_mesh_common::interfaces::server_api::{
    GetTokenRequest, GetTokenResponse, GetTokenResponseEnum, GetTokenResponseMap,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use dashmap::try_result::TryResult::Present;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use sqlx::PgPool;

#[tracing::instrument(name = "get_token", skip_all)]
pub async fn get_token(
    Extension(enable_caching): Extension<bool>,
    Extension(pool): Extension<PgPool>,
    Extension(get_token_map): Extension<GetTokenResponseMap>,
    Json(body): Json<GetTokenRequest>,
) -> Result<impl IntoResponse, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.password.clone());
    if enable_caching {
        if let Present(entry) = get_token_map.try_get(&key) {
            return match entry.value() {
                GetTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone()).into_response()),
                GetTokenResponseEnum::UserNotFound => {
                    Ok((StatusCode::NO_CONTENT, "User Not Found").into_response())
                }
                GetTokenResponseEnum::PasswordMismatch => {
                    Ok((StatusCode::NO_CONTENT, "Password Mismatch").into_response())
                }
                GetTokenResponseEnum::ApiTokenNotFound => {
                    Ok((StatusCode::NO_CONTENT, "Api Token Not Found").into_response())
                }
            };
        }
    }
    let mut transaction = create_txn(&pool).await?;
    let user = match get_user_and_api_token_by_email(&mut transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                commit_txn(transaction).await?;
                get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
                return Ok((StatusCode::NO_CONTENT, "User Not Found").into_response());
            }
        },
        Err(_) => {
            commit_txn(transaction).await?;
            get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
            return Ok((StatusCode::NO_CONTENT, "User Not Found").into_response());
        }
    };
    if !verify::<&str>(body.password.as_ref(), user.password.as_ref()).unwrap_or(false) {
        commit_txn(transaction).await?;
        get_token_map.insert(key, GetTokenResponseEnum::PasswordMismatch);
        return Ok((StatusCode::NO_CONTENT, "Password Mismatch").into_response());
    }
    let response = GetTokenResponse {
        api_token: Some(*user.token.as_ref()),
        message: None,
    };
    commit_txn(transaction).await?;
    get_token_map.insert(
        key,
        GetTokenResponseEnum::GetTokenResponse(response.clone()),
    );
    Ok(Json(response).into_response())
}
