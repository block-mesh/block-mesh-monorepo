use std::env;
use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use bcrypt::{hash, DEFAULT_COST};
use block_mesh_common::constants::{DeviceType, BLOCK_MESH_EMAILS};
use block_mesh_common::interfaces::server_api::{RegisterForm, SendEmail};
use block_mesh_common::reqwest::http_client;
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::nonce::Nonce;
use block_mesh_manager_database_domain::domain::prep_user::prep_user;
use chrono::{Duration, Utc};
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::HeaderMap;
use secret::Secret;
use sqlx::PgPool;
use tokio::sync::OnceCell;
use uuid::Uuid;
use validator::validate_email;

use crate::database::api_token::create_api_token::create_api_token;
use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::database::invite_code::get_user_opt_by_invited_code::get_user_opt_by_invited_code;
use crate::database::nonce::create_nonce::create_nonce;
use crate::database::uptime_report::create_uptime_report::create_uptime_report;
use crate::database::user::create_user::create_user;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_invited_by::update_user_invited_by;
use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use crate::utils::cftoken::check_cf_token;

static RATE_LIMIT_IP: OnceCell<DashSetWithExpiry<String>> = OnceCell::const_new();

#[tracing::instrument(name = "register_post", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Form(form): Form<RegisterForm>,
) -> Result<Redirect, Error> {
    let email = form.email.clone().to_ascii_lowercase();
    let app_env = get_envar("APP_ENVIRONMENT").await;
    let header_ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    }
    .to_string();

    let cache = RATE_LIMIT_IP
        .get_or_init(|| async { DashSetWithExpiry::new() })
        .await;
    if cache.get(&header_ip).is_some() || cache.get(&email).is_some() {
        return Err(Error::NotAllowedRateLimit);
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    cache.insert(header_ip, Some(date));
    cache.insert(email.clone(), Some(date));

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

    if !validate_email(email.clone()) {
        return Ok(Error::redirect(
            400,
            "Invalid email",
            "Please check if email you inserted is correct",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }

    if form.password.contains(' ') {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password cannot contain spaces",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    } else if form.password.chars().all(char::is_alphanumeric) {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password must contain a special characters",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    } else if form.password.len() < 8 {
        return Ok(Error::redirect(
            400,
            "Invalid Password",
            "Password must be at least 8 characters long",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }

    if form.password_confirm != form.password {
        return Ok(Error::redirect(
            400,
            "Password Mismatch",
            "Please check if your password and password confirm are the same",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }

    let user = get_user_opt_by_email(&mut transaction, &email).await?;
    if user.is_some() {
        return Ok(Error::redirect(
            400,
            "User Already Exists",
            "User with this email already exists",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
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
                    RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
                ))
            }
        }
    } else {
        return Ok(Error::redirect(
            400,
            "Invite Code Not Found",
            "Please add an invite code",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
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
    let email_mode = get_envar("EMAIL_MODE").await;
    if email_mode == "AWS" {
        state
            .email_client
            .send_confirmation_email_aws(&email, nonce_secret.expose_secret())
            .await?;
    } else if email_mode == "SERVICE" {
        let client = http_client(DeviceType::AppServer);
        client
            .get(format!("{}/send_email", BLOCK_MESH_EMAILS))
            .query(&SendEmail {
                code: env::var("EMAILS_CODE").unwrap_or_default(),
                user_id: session.id,
                email_type: "confirm_email".to_string(),
                email_address: session.email,
                nonce: session.nonce,
            })
            .send()
            .await
            .map_err(Error::from)?;
    } else {
        state
            .email_client
            .send_confirmation_email_smtp(&email, nonce_secret.expose_secret())
            .await?;
    }
    Ok(Redirect::to("/ui/dashboard"))
}
