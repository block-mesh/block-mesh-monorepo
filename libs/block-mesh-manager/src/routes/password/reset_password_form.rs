use askama::Template;
use askama_axum::IntoResponse;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};

use crate::middlewares::authentication::Backend;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "reset_password.html")]
struct ResetPasswordTemplate {
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

#[tracing::instrument(name = "password")]
pub async fn handler(
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Redirect> {
    return match auth.user {
        Some(_) => Err(Redirect::to("/dashboard")),
        None => Ok(ResetPasswordTemplate {
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
    };
}
