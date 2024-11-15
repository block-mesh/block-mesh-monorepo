use crate::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, CheckTokenResponseEnum, CheckTokenResponseMap, GetTokenResponse,
};
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;

#[tracing::instrument(name = "check_token", skip_all)]
pub async fn check_token(
    Extension(enable_caching): Extension<bool>,
    Extension(pool): Extension<PgPool>,
    Extension(check_token_map): Extension<CheckTokenResponseMap>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.api_token);

    if enable_caching {
        if let Some(entry) = check_token_map.get(&key) {
            return match entry.value() {
                CheckTokenResponseEnum::ApiTokenMismatch => Err(Error::ApiTokenMismatch),
                CheckTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
                CheckTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
                CheckTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
            };
        }
    }

    let mut transaction = create_txn(&pool).await?;

    // let user = match get_user_opt_by_email(&mut *transaction, &email).await {
    let user = match get_user_and_api_token_by_email(&mut transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                commit_txn(transaction).await?;
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            commit_txn(transaction).await?;
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            return Err(Error::UserNotFound);
        }
    };
    // let api_token =
    //     match get_api_token_by_usr_and_status(&pool, &user.id, ApiTokenStatus::Active).await {
    //         Ok(api_token) => match api_token {
    //             Some(api_token) => api_token,
    //             None => {
    //                 check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
    //                 commit_txn(transaction).await?;
    //                 return Err(Error::ApiTokenNotFound);
    //             }
    //         },
    //         Err(_) => {
    //             check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
    //             commit_txn(transaction).await?;
    //             return Err(Error::ApiTokenNotFound);
    //         }
    //     };

    if *user.token.as_ref() != body.api_token {
        commit_txn(transaction).await?;
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
        return Err(Error::ApiTokenMismatch);
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
