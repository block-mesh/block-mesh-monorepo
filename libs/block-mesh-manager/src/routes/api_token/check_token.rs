use crate::database::user::get_user_and_api_token::get_user_and_api_token_by_email;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, CheckTokenResponseEnum, GetTokenResponse,
};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "check_token", skip(pool, state))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.api_token);
    let check_token_map = &state.check_token_map;

    if let Some(entry) = check_token_map.get(&key) {
        return match entry.value() {
            CheckTokenResponseEnum::ApiTokenMismatch => Err(Error::ApiTokenMismatch),
            CheckTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
            CheckTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
            CheckTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
        };
    }

    let mut transaction = create_txn(&pool).await?;
    let user_and_api_token = match get_user_and_api_token_by_email(&mut transaction, &email).await {
        Ok(user_and_api_token) => match user_and_api_token {
            Some(user) => user,
            None => {
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                commit_txn(transaction).await?;
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            commit_txn(transaction).await?;
            return Err(Error::UserNotFound);
        }
    };
    // let api_token =
    //     match get_api_token_by_usr_and_status(&mut trasaction, &user.id, ApiTokenStatus::Active)
    //         .await
    //     {
    //         Ok(api_token) => match api_token {
    //             Some(api_token) => api_token,
    //             None => {
    //                 check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
    //                 commit_txn(trasaction).await?;
    //                 return Err(Error::ApiTokenNotFound);
    //             }
    //         },
    //         Err(_) => {
    //             check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
    //             commit_txn(trasaction).await?;
    //             return Err(Error::ApiTokenNotFound);
    //         }
    //     };
    //
    if *user_and_api_token.token.as_ref() != body.api_token {
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
        commit_txn(transaction).await?;
        return Err(Error::ApiTokenMismatch);
    }

    let response = GetTokenResponse {
        api_token: Some(*user_and_api_token.token.as_ref()),
        message: None,
    };
    check_token_map.insert(
        key,
        CheckTokenResponseEnum::GetTokenResponse(response.clone()),
    );

    commit_txn(transaction).await?;
    Ok(Json(response))
}
