use crate::errors::error::Error;
use crate::middlewares::authentication::{Backend, Credentials};
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::LoginForm;
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::create_daily_stat::get_or_create_daily_stat;
use block_mesh_manager_database_domain::domain::get_user_and_api_token_by_email::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::touch_user_aggregates::touch_user_aggregates;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use secret::Secret;
use sqlx::PgPool;

#[tracing::instrument(name = "login_post", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Form(form): Form<LoginForm>,
) -> Result<Redirect, Error> {
    let mut transaction = create_txn(&pool).await?;
    let user = get_user_and_api_token_by_email(&mut transaction, &form.email.to_ascii_lowercase())
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let _ = get_or_create_daily_stat(&mut transaction, &user.user_id, None).await?;
    let _ = touch_user_aggregates(&mut transaction, &user.user_id).await;
    commit_txn(transaction).await?;
    let creds: Credentials = Credentials {
        email: form.email.to_ascii_lowercase(),
        password: Secret::from(form.password),
        nonce: user.nonce.as_ref().to_string(),
    };
    let session = match auth.authenticate(creds).await {
        Ok(Some(user)) => user,
        _ => {
            return Ok(Error::redirect(
                400,
                "Authentication failed",
                "Authentication failed. Please try again or reset password https://app.blockmesh.xyz/reset_password",
                RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
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
                "Login failed. Please try again  or reset password https://app.blockmesh.xyz/reset_password",
                RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
            ));
        }
    }
    Ok(Redirect::to("/ui/dashboard"))
}
