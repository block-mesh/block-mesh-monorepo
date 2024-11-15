use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{LoginWalletForm, SigArray};
use block_mesh_common::routes_enum::RoutesEnum;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use redis::{AsyncCommands, RedisResult};
use secret::Secret;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;

#[tracing::instrument(name = "login_wallet_post", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<LoginWalletForm>,
) -> Result<Redirect, Error> {
    let mut redis = state.redis.clone();
    let mut transaction = create_txn(&pool).await?;
    let pubkey = Pubkey::from_str(form.pubkey.as_str()).unwrap_or_default();
    let email = format!("wallet_{pubkey}@blockmesh.xyz").to_ascii_lowercase();
    tracing::info!("form = {:#?}", form);
    tracing::info!("email = {:#?}", email);

    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let creds: Credentials = Credentials {
        email,
        password: Secret::from(form.password),
        nonce: nonce.nonce.as_ref().to_string(),
    };
    let sig_array: SigArray = match serde_json::from_str(&form.signature) {
        Ok(sig_array) => sig_array,
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Error",
                "Bad Signature",
                RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
            ));
        }
    };
    let signature =
        Signature::try_from(sig_array.0.as_slice()).map_err(|_| Error::InternalServer)?;

    let form_nonce = form.nonce.clone();
    let message = form.nonce.as_bytes();
    let redis_nonce: RedisResult<String> = redis.get(form_nonce.clone()).await;
    match redis_nonce {
        Ok(redis_nonce) => {
            if redis_nonce != form_nonce {
                return Ok(Error::redirect(
                    400,
                    "Retry please",
                    "Invalid nonce",
                    RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
                ));
            }
        }
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Retry please",
                "Missing nonce",
                RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
            ));
        }
    }

    if !signature.verify(pubkey.as_ref(), message) {
        tracing::error!("Signature verification failed.");
        return Err(Error::SignatureMismatch);
    }
    let session = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        _ => {
            commit_txn(transaction).await?;
            return Ok(Error::redirect(
                400,
                "Authentication failed",
                "Authentication failed. Please try again.",
                RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
            ));
        }
    };

    match auth.login(&session).await {
        Ok(_) => {}
        Err(e) => {
            commit_txn(transaction).await?;
            tracing::error!("Login failed: {:?} for user {}", e, user.id);
            return Ok(Error::redirect(
                400,
                "Login Failed",
                "Login failed. Please try again.",
                RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
            ));
        }
    }
    commit_txn(transaction).await?;
    Ok(Redirect::to("/ui/dashboard"))
}
