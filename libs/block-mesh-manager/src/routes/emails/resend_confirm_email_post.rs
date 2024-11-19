use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::ResendConfirmEmailForm;
use block_mesh_common::routes_enum::RoutesEnum;
use chrono::Duration;
use chrono::Utc;
use dash_with_expiry::dash_set_with_expiry::DashSetWithExpiry;
use std::sync::Arc;
use tokio::sync::OnceCell;
static RATE_LIMIT_EMAIL: OnceCell<DashSetWithExpiry<String>> = OnceCell::const_new();

pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
    State(state): State<Arc<AppState>>,
    Form(_form): Form<ResendConfirmEmailForm>,
) -> Result<Redirect, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let cache = RATE_LIMIT_EMAIL
        .get_or_init(|| async { DashSetWithExpiry::new() })
        .await;
    if let Some(_) = cache.get(&user.email) {
        return Err(Error::NotAllowedRateLimit);
    }
    let email_mode = get_envar("EMAIL_MODE").await;
    if email_mode == "AWS" {
        state
            .email_client
            .send_confirmation_email_aws(&user.email, &user.nonce)
            .await?;
    } else {
        state
            .email_client
            .send_confirmation_email_gmail(&user.email, &user.nonce)
            .await?;
    }
    let date = Utc::now() + Duration::milliseconds(60_000);
    cache.insert(user.email, Some(date));
    Ok(NotificationRedirect::redirect(
        "Email Sent",
        "Please check your email",
        RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
    ))
}
