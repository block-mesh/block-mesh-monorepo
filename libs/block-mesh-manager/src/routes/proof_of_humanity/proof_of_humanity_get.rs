use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "proof_of_humanity.html")]
struct ProofOfHumanTemplate {
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
    pub cf_site_key: String,
    pub recaptcha_site_key_v2: String,
    pub recaptcha_site_key_v3: String,
    pub hcaptcha_site_key: String,
    pub enable_hcaptcha: bool,
    pub enable_recaptcha: bool,
}

#[tracing::instrument(name = "proof_of_humanity_get", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let _ = auth.user.ok_or(Error::UserNotFound)?;
    Ok(ProofOfHumanTemplate {
        enable_recaptcha: state.enable_recaptcha,
        enable_hcaptcha: state.enable_hcaptcha,
        hcaptcha_site_key: state.hcaptcha_site_key.clone(),
        recaptcha_site_key_v2: state.recaptcha_site_key_v2.clone(),
        recaptcha_site_key_v3: state.recaptcha_site_key_v3.clone(),
        cf_site_key: state.cf_site_key.clone(),
        chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
        app_server: BLOCK_MESH_APP_SERVER.to_string(),
        github: BLOCK_MESH_GITHUB.to_string(),
        twitter: BLOCK_MESH_TWITTER.to_string(),
        gitbook: BLOCK_MESH_GITBOOK.to_string(),
        logo: BLOCK_MESH_LOGO.to_string(),
        image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
        support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
        chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
    })
}
