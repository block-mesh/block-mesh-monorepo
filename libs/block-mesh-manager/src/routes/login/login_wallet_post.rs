use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{LoginWalletForm, SigArray};
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::create_daily_stat::get_or_create_daily_stat;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::touch_user_aggregates::touch_user_aggregates;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use secret::Secret;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;

#[tracing::instrument(name = "login_wallet_post", skip_all)]
pub async fn handler(
    State(_state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<LoginWalletForm>,
) -> Result<Redirect, Error> {
    let pubkey = Pubkey::from_str(form.pubkey.as_str()).unwrap_or_default();
    let email = format!("wallet_{pubkey}@blockmesh.xyz").to_ascii_lowercase();
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &email.to_ascii_lowercase())
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let _ = get_or_create_daily_stat(&mut transaction, &user.user_id, None).await?;
    let _ = touch_user_aggregates(&mut transaction, &user.user_id).await;
    commit_txn(transaction).await?;
    let creds: Credentials = Credentials {
        email: email.to_ascii_lowercase(),
        password: Secret::from(form.password),
        nonce: user.nonce.as_ref().to_string(),
    };
    let user_wallet = user.wallet_address.unwrap_or_default().to_lowercase();
    let form_wallet = form.pubkey.to_lowercase();
    if user_wallet != form_wallet {
        return Ok(Error::redirect(
            400,
            "Error",
            "Wallet Mismatch",
            RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
        ));
    }
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
    let message = form.nonce.as_bytes();
    if !signature.verify(pubkey.as_ref(), message) {
        tracing::error!("Signature verification failed.");
        return Err(Error::SignatureMismatch);
    }
    let session = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        _ => {
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
            tracing::error!("Login failed: {:?} for user {}", e, user.user_id);
            return Ok(Error::redirect(
                400,
                "Login Failed",
                "Login failed. Please try again.",
                RoutesEnum::Static_UnAuth_Login_Wallet.to_string().as_str(),
            ));
        }
    }
    Ok(Redirect::to("/ui/dashboard"))
}
