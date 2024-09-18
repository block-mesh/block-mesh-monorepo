use crate::database::get_api_token_by_usr_and_status::get_api_token_by_usr_and_status_pool;
use crate::database::get_user_opt_by_email::get_user_opt_by_email;
use crate::error::Error;
use axum::{Extension, Json};
use bcrypt::verify;
use block_mesh_common::interfaces::server_api::{GetTokenRequest, GetTokenResponse};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type GetTokenResponseMap = Arc<Mutex<HashMap<(String, String), GetTokenResponseEnum>>>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum GetTokenResponseEnum {
    GetTokenResponse(GetTokenResponse),
    UserNotFound,
    PasswordMismatch,
    ApiTokenNotFound,
}

pub async fn get_token(
    Extension(pool): Extension<PgPool>,
    Extension(get_token_map): Extension<GetTokenResponseMap>,
    Json(body): Json<GetTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let mut transaction = pool.begin().await?;
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.password.clone());
    let mut get_token_map = get_token_map.lock().await;

    if let Some(value) = get_token_map.get(&key) {
        return match value {
            GetTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
            GetTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
            GetTokenResponseEnum::PasswordMismatch => Err(Error::PasswordMismatch),
            GetTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
        };
    }

    let user = match get_user_opt_by_email(&mut transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
            return Err(Error::UserNotFound);
        }
    };

    if !verify::<&str>(body.password.as_ref(), user.password.as_ref()).unwrap_or(false) {
        get_token_map.insert(key, GetTokenResponseEnum::PasswordMismatch);
        return Err(Error::PasswordMismatch);
    }

    let api_token =
        match get_api_token_by_usr_and_status_pool(&pool, &user.id, ApiTokenStatus::Active).await {
            Ok(api_token) => match api_token {
                Some(api_token) => api_token,
                None => {
                    get_token_map.insert(key, GetTokenResponseEnum::ApiTokenNotFound);
                    return Err(Error::ApiTokenNotFound);
                }
            },
            Err(_) => {
                get_token_map.insert(key, GetTokenResponseEnum::ApiTokenNotFound);
                return Err(Error::ApiTokenNotFound);
            }
        };

    let response = GetTokenResponse {
        api_token: Some(*api_token.token.as_ref()),
        message: None,
    };

    get_token_map.insert(
        key,
        GetTokenResponseEnum::GetTokenResponse(response.clone()),
    );
    Ok(Json(response))
}
