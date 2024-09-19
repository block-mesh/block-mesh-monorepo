use crate::database::get_api_token_by_usr_and_status::get_api_token_by_usr_and_status;
use crate::database::get_user_opt_by_email::get_user_opt_by_email;
use crate::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{CheckTokenRequest, GetTokenResponse};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub type CheckTokenResponseMap = Arc<DashMap<(String, Uuid), CheckTokenResponseEnum>>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CheckTokenResponseEnum {
    GetTokenResponse(GetTokenResponse),
    UserNotFound,
    ApiTokenMismatch,
    ApiTokenNotFound,
}

#[tracing::instrument(name = "check_token", skip_all)]
pub async fn check_token(
    Extension(pool): Extension<PgPool>,
    Extension(check_token_map): Extension<CheckTokenResponseMap>,
    Json(body): Json<CheckTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.api_token);

    if let Some(entry) = check_token_map.get(&key) {
        return match entry.value() {
            CheckTokenResponseEnum::ApiTokenMismatch => Err(Error::ApiTokenMismatch),
            CheckTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
            CheckTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
            CheckTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
        };
    }

    let user = match get_user_opt_by_email(&pool, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            return Err(Error::UserNotFound);
        }
    };
    let api_token =
        match get_api_token_by_usr_and_status(&pool, &user.id, ApiTokenStatus::Active).await {
            Ok(api_token) => match api_token {
                Some(api_token) => api_token,
                None => {
                    check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                    return Err(Error::ApiTokenNotFound);
                }
            },
            Err(_) => {
                check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                return Err(Error::ApiTokenNotFound);
            }
        };

    if *api_token.token.as_ref() != body.api_token {
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
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

    Ok(Json(response))
}
