use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::Extension;
use axum_login::AuthSession;
use sqlx::PgPool;

use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use block_mesh_common::interfaces::server_api::NewPasswordQuery;

use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce_pool;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "new_password.html")]
struct NewPasswordTemplate {
    pub email: String,
    pub token: String,
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
}

#[tracing::instrument(name = "new_password")]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Query(query): Query<NewPasswordQuery>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let token = query.token;
    let nonce = get_nonce_by_nonce_pool(&pool, &token)
        .await?
        .ok_or_else(|| Error::NonceNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &nonce.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    match auth.user {
        Some(_) => Err(Error::UserAlreadyExists),
        None => Ok(NewPasswordTemplate {
            email: user.email.to_ascii_lowercase(),
            token,
            chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
            app_server: BLOCK_MESH_APP_SERVER.to_string(),
            github: BLOCK_MESH_GITHUB.to_string(),
            twitter: BLOCK_MESH_TWITTER.to_string(),
            gitbook: BLOCK_MESH_GITBOOK.to_string(),
            logo: BLOCK_MESH_LOGO.to_string(),
            image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
            support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
            chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
        }),
    }
}
