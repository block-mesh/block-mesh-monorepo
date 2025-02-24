use std::str::FromStr;
use std::sync::Arc;

use crate::database::api_token::update_api_token::update_api_token;
use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::user::get_user_by_wallet::get_user_opt_by_wallet;
use crate::database::user::update_user_password::update_user_password;
use crate::database::user::update_user_wallet::update_user_wallet;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use crate::utils::cftoken::check_cf_token;
use anyhow::anyhow;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::interfaces::server_api::{ResetPasswordWalletForm, SigArray};
use block_mesh_common::routes_enum::RoutesEnum;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use secret::Secret;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;

#[tracing::instrument(name = "register_wallet_post", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<ResetPasswordWalletForm>,
) -> Result<Redirect, Error> {
    if state.cf_enforce {
        if let Err(e) = check_cf_token(form.cftoken.unwrap_or_default(), &state.cf_secret_key).await
        {
            return Ok(Error::redirect(
                400,
                "Error in human validation",
                &format!("The following error occurred: {}", e),
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ));
        }
    }
    let mut transaction = create_txn(&pool).await?;
    let sig_array: SigArray = match serde_json::from_str(&form.signature) {
        Ok(sig_array) => sig_array,
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Error",
                "Bad Signature",
                RoutesEnum::Static_UnAuth_Register_Wallet
                    .to_string()
                    .as_str(),
            ));
        }
    };
    let signature =
        Signature::try_from(sig_array.0.as_slice()).map_err(|_| Error::InternalServer)?;
    let pubkey = Pubkey::from_str(form.pubkey.as_str()).unwrap_or_default();
    let form_nonce = form.nonce.clone();
    let message = form.nonce.as_bytes();
    let mem_nonce = state.wallet_login_nonce.get(&form_nonce).await;
    match mem_nonce {
        Some(mem_nonce) => {
            if mem_nonce != form_nonce {
                return Ok(Error::redirect(
                    400,
                    "Retry please",
                    "Invalid nonce",
                    RoutesEnum::Static_UnAuth_Register_Wallet
                        .to_string()
                        .as_str(),
                ));
            }
        }
        None => {
            return Ok(Error::redirect(
                400,
                "Retry please",
                "Missing nonce",
                RoutesEnum::Static_UnAuth_Register_Wallet
                    .to_string()
                    .as_str(),
            ));
        }
    }
    if form.password.contains(' ') {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password cannot contain spaces",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    } else if form.password.chars().all(char::is_alphanumeric) {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password must contain a special characters",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    } else if form.password.len() < 8 {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password must be at least 8 characters long",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    }

    if form.password_confirm != form.password {
        return Ok(Error::redirect(
            400,
            "Password Mismatch",
            "Please check if your password and password confirm are the same",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    }

    let user = get_user_opt_by_wallet(&mut transaction, &form.pubkey).await?;
    if user.is_none() {
        return Ok(Error::redirect(
            400,
            "User Doesnt Exists",
            "User with this wallet doesnt",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    }

    let user = user.unwrap();
    let user_id = user.id;
    let hashed_password = hash(form.password.clone(), DEFAULT_COST)?;
    let email = user.email;
    let nonce = get_nonce_by_user_id(&mut transaction, &user_id).await?;
    if nonce.is_none() {
        return Ok(Error::redirect(
            400,
            "Missing Nonce",
            "Missing User Nonce",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    }
    let nonce = nonce.unwrap().nonce.expose_secret().to_string();
    update_user_password(&mut transaction, user.id, hashed_password).await?;
    update_api_token(&mut transaction, user.id).await?;

    if signature.verify(pubkey.as_ref(), message) {
        add_perk_to_user(
            &mut transaction,
            user_id,
            PerkName::Wallet,
            1.1,
            0.0,
            serde_json::from_str("{}").unwrap(),
        )
        .await?;
        update_user_wallet(&mut transaction, user_id, &form.pubkey).await?;
        state
            .wallet_addresses
            .insert(email.clone(), Some(form.pubkey), None)
            .await;
    } else {
        tracing::error!("Signature verification failed.");
        return Err(Error::SignatureMismatch);
    }
    commit_txn(transaction).await?;
    let creds: Credentials = Credentials {
        email: email.clone(),
        password: Secret::from(form.password),
        nonce,
    };
    let session = auth
        .authenticate(creds)
        .await
        .map_err(|_| Error::Auth(anyhow!("Authentication failed").to_string()))?
        .ok_or(Error::UserNotFound)?;
    auth.login(&session)
        .await
        .map_err(|_| Error::Auth(anyhow!("Login failed").to_string()))?;
    Ok(Redirect::to("/ui/dashboard"))
}
