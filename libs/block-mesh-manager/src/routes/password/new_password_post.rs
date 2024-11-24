use crate::database::api_token::update_api_token::update_api_token;
use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_password::update_user_password;
use crate::domain::password::Password;
use crate::errors::error::Error;
use crate::middlewares::authentication::del_from_redis_with_pattern;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::interfaces::db_messages::InvalidateApiCache;
use block_mesh_common::interfaces::server_api::NewPasswordForm;
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::notify_api::notify_api;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Form(form): Form<NewPasswordForm>,
) -> Result<Redirect, Error> {
    let mut redis = state.redis.clone();
    let mut transaction = match create_txn(&pool).await {
        Ok(transaction) => transaction,
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Server Error",
                "Please retry",
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ))
        }
    };
    let email = form.email.clone().to_ascii_lowercase();
    if form.password_confirm != form.password {
        return Ok(Error::redirect(
            400,
            "Password Mismatch",
            "Please check if your password and password confirm are the same",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }
    if let Err(e) = Password::new(form.password.clone()) {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            &e.to_string(),
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    if *nonce.nonce.expose_secret() != form.token {
        return Err(Error::TokenMismatch);
    }
    let hashed_password = hash(form.password.clone(), DEFAULT_COST)?;
    update_user_password(&mut transaction, user.id, hashed_password).await?;
    update_api_token(&mut transaction, user.id).await?;
    match commit_txn(transaction).await {
        Ok(_) => {}
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Server Error",
                "Please resubmit",
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ))
        }
    };
    let keys: Vec<(String, String)> = state
        .get_token_map
        .iter()
        .find(|r| {
            let key = r.key();
            key.0 == user.email
        })
        .iter()
        .map(|i| i.key().clone())
        .collect();
    keys.iter().for_each(|key| {
        state.get_token_map.remove(key);
    });

    let keys: Vec<(String, Uuid)> = state
        .check_token_map
        .iter()
        .find(|r| {
            let key = r.key();
            key.0 == user.email
        })
        .iter()
        .map(|i| i.key().clone())
        .collect();
    keys.iter().for_each(|key| {
        state.check_token_map.remove(key);
    });
    del_from_redis_with_pattern(&email, "-*", &mut redis).await?;
    del_from_redis_with_pattern(&user.id.to_string(), "-*", &mut redis).await?;
    let _ = notify_api(
        &state.channel_pool,
        InvalidateApiCache { email: user.email },
    )
    .await;
    Ok(NotificationRedirect::redirect(
        "Password updated",
        "Please use the new password and login",
        RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
    ))
}
