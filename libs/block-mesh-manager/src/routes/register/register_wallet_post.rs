use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use bcrypt::{hash, DEFAULT_COST};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;
use uuid::Uuid;

use block_mesh_common::interfaces::server_api::{RegisterWalletForm, SigArray};
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::nonce::Nonce;
use block_mesh_manager_database_domain::domain::prep_user::prep_user;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use secret::Secret;

use crate::database::api_token::create_api_token::create_api_token;
use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::database::invite_code::get_user_opt_by_invited_code::get_user_opt_by_invited_code;
use crate::database::nonce::create_nonce::create_nonce;
use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::user::create_user::create_user;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_invited_by::update_user_invited_by;
use crate::database::user::update_user_wallet::update_user_wallet;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use crate::utils::cftoken::check_cf_token;

#[tracing::instrument(name = "register_wallet_post", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterWalletForm>,
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
    let email = format!("wallet_{pubkey}@blockmesh.xyz").to_ascii_lowercase();
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

    let user = get_user_opt_by_email(&mut transaction, &email).await?;
    if user.is_some() {
        return Ok(Error::redirect(
            400,
            "User Already Exists",
            "User with this email already exists",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
    }

    let nonce = Nonce::generate_nonce(16);
    let nonce_secret = Secret::from(nonce.clone());
    let hashed_password = hash(form.password.clone(), DEFAULT_COST)?;
    let user_id = create_user(&mut transaction, None, &email, &hashed_password)
        .await
        .map_err(Error::from)?;
    create_nonce(&mut transaction, &user_id, &nonce_secret).await?;
    create_api_token(&mut transaction, user_id).await?;
    create_invite_code(&mut transaction, user_id, &Uuid::new_v4().to_string()).await?;
    create_uptime_report(&mut transaction, &user_id, &None).await?;
    prep_user(&mut transaction, &user_id).await?;

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

    if !form.invite_code.is_empty() {
        match get_user_opt_by_invited_code(&mut transaction, form.invite_code).await? {
            Some(invited_by_user) => {
                let invited_by_user_id = invited_by_user.user_id;
                update_user_invited_by(&mut transaction, user_id, invited_by_user_id).await?;
            }
            None => {
                return Ok(Error::redirect(
                    400,
                    "Invite Code Not Found",
                    "Please check if the invite you insert is correct",
                    RoutesEnum::Static_UnAuth_Register_Wallet
                        .to_string()
                        .as_str(),
                ))
            }
        }
    } else {
        return Ok(Error::redirect(
            400,
            "Invite Code Not Found",
            "Please add an invite code",
            RoutesEnum::Static_UnAuth_Register_Wallet
                .to_string()
                .as_str(),
        ));
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
