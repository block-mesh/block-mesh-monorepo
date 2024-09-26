use crate::database::get_api_token_by_usr_and_status::get_api_token_by_usr_and_status;
use crate::database::get_user_opt_by_email::get_user_opt_by_email;
use crate::error::Error;
use anyhow::Context;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{
    CheckTokenRequest, CheckTokenResponseEnum, CheckTokenResponseMap, GetTokenResponse,
};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use sqlx::PgPool;

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

    let mut transaction = pool.begin().await?;

    let user = match get_user_opt_by_email(&mut *transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
                let _ = transaction.commit().await.context("Cannot commit txn");
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            check_token_map.insert(key, CheckTokenResponseEnum::UserNotFound);
            let _ = transaction.commit().await.context("Cannot commit txn");
            return Err(Error::UserNotFound);
        }
    };
    let api_token =
        match get_api_token_by_usr_and_status(&pool, &user.id, ApiTokenStatus::Active).await {
            Ok(api_token) => match api_token {
                Some(api_token) => api_token,
                None => {
                    check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                    let _ = transaction.commit().await.context("Cannot commit txn");
                    return Err(Error::ApiTokenNotFound);
                }
            },
            Err(_) => {
                check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenNotFound);
                let _ = transaction.commit().await.context("Cannot commit txn");
                return Err(Error::ApiTokenNotFound);
            }
        };

    if *api_token.token.as_ref() != body.api_token {
        check_token_map.insert(key, CheckTokenResponseEnum::ApiTokenMismatch);
        let _ = transaction.commit().await.context("Cannot commit txn");
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
    let _ = transaction.commit().await.context("Cannot commit txn");

    Ok(Json(response))
}
