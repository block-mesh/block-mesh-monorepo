use crate::error::Error;
use axum::{Extension, Json};
use bcrypt::verify;
use block_mesh_common::interfaces::server_api::{
    GetTokenRequest, GetTokenResponse, GetTokenResponseEnum, GetTokenResponseMap,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;

#[tracing::instrument(name = "get_token", skip_all)]
pub async fn get_token(
    Extension(enable_caching): Extension<bool>,
    Extension(pool): Extension<PgPool>,
    Extension(get_token_map): Extension<GetTokenResponseMap>,
    Json(body): Json<GetTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.password.clone());
    if enable_caching {
        if let Some(entry) = get_token_map.get(&key) {
            return match entry.value() {
                GetTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
                GetTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
                GetTokenResponseEnum::PasswordMismatch => Err(Error::PasswordMismatch),
                GetTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
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
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            commit_txn(transaction).await?;
            get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
            return Err(Error::UserNotFound);
        }
    };
    if !verify::<&str>(body.password.as_ref(), user.password.as_ref()).unwrap_or(false) {
        commit_txn(transaction).await?;
        get_token_map.insert(key, GetTokenResponseEnum::PasswordMismatch);
        return Err(Error::PasswordMismatch);
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
    Ok(Json(response))
}
