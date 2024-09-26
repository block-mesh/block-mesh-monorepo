use crate::database::get_api_token_by_usr_and_status::get_api_token_by_usr_and_status;
use crate::database::get_user_opt_by_email::get_user_opt_by_email;
use crate::error::Error;
use anyhow::Context;
use axum::{Extension, Json};
use bcrypt::verify;
use block_mesh_common::interfaces::server_api::{
    GetTokenRequest, GetTokenResponse, GetTokenResponseEnum, GetTokenResponseMap,
};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use sqlx::PgPool;

#[tracing::instrument(name = "get_token", skip_all)]
pub async fn get_token(
    Extension(pool): Extension<PgPool>,
    Extension(get_token_map): Extension<GetTokenResponseMap>,
    Json(body): Json<GetTokenRequest>,
) -> Result<Json<GetTokenResponse>, Error> {
    let email = body.email.clone().to_ascii_lowercase();
    let key = (email.clone(), body.password.clone());

    if let Some(entry) = get_token_map.get(&key) {
        return match entry.value() {
            GetTokenResponseEnum::GetTokenResponse(r) => Ok(Json(r.clone())),
            GetTokenResponseEnum::UserNotFound => Err(Error::UserNotFound),
            GetTokenResponseEnum::PasswordMismatch => Err(Error::PasswordMismatch),
            GetTokenResponseEnum::ApiTokenNotFound => Err(Error::ApiTokenNotFound),
        };
    }

    let mut transaction = pool.begin().await?;

    let user = match get_user_opt_by_email(&mut *transaction, &email).await {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                let _ = transaction.commit().await.context("Cannot commit txn");
                get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
                return Err(Error::UserNotFound);
            }
        },
        Err(_) => {
            get_token_map.insert(key, GetTokenResponseEnum::UserNotFound);
            let _ = transaction.commit().await.context("Cannot commit txn");
            return Err(Error::UserNotFound);
        }
    };

    if !verify::<&str>(body.password.as_ref(), user.password.as_ref()).unwrap_or(false) {
        get_token_map.insert(key, GetTokenResponseEnum::PasswordMismatch);
        let _ = transaction.commit().await.context("Cannot commit txn");
        return Err(Error::PasswordMismatch);
    }

    let api_token =
        match get_api_token_by_usr_and_status(&mut *transaction, &user.id, ApiTokenStatus::Active)
            .await
        {
            Ok(api_token) => match api_token {
                Some(api_token) => api_token,
                None => {
                    get_token_map.insert(key, GetTokenResponseEnum::ApiTokenNotFound);
                    let _ = transaction.commit().await.context("Cannot commit txn");
                    return Err(Error::ApiTokenNotFound);
                }
            },
            Err(_) => {
                get_token_map.insert(key, GetTokenResponseEnum::ApiTokenNotFound);
                let _ = transaction.commit().await.context("Cannot commit txn");
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
    let _ = transaction.commit().await.context("Cannot commit txn");
    Ok(Json(response))
}
