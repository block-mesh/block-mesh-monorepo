use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, CheckTokenResponseEnum, GetTokenResponse,
};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
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

    let mut trasaction = create_txn(&pool).await?;

    let user = match get_user_opt_by_email(&mut trasaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                commit_txn(trasaction).await?;
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            commit_txn(trasaction).await?;
            return Err(Error::UserNotFound);
        }
    };
    let api_token =
        match get_api_token_by_usr_and_status(&mut trasaction, &user.id, ApiTokenStatus::Active)
            .await
        {
            Ok(api_token) => match api_token {
                Some(api_token) => api_token,
                None => {
                    check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                    commit_txn(trasaction).await?;
                    return Err(Error::ApiTokenNotFound);
                }
            },
            Err(_) => {
                check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                commit_txn(trasaction).await?;
                return Err(Error::ApiTokenNotFound);
            }
        };

    if *api_token.token.as_ref() != body.api_token {
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
        commit_txn(trasaction).await?;
        return Err(Error::ApiTokenMismatch);
    }

    let response = GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    };
    check_token_map.insert(
        key,
        CheckTokenResponseEnum::GetTokenResponse(response.clone()),
    );

    commit_txn(trasaction).await?;
    Ok(Json(response))
}
