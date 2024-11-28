use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use block_mesh_manager_database_domain::domain::nonce::Nonce;
use redis::{AsyncCommands, RedisResult};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "login_wallet.html")]
struct LoginTemplate {
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
    pub nonce: String,
    pub cf_site_key: String,
}

#[tracing::instrument(name = "login_wallet", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Redirect> {
    match auth.user {
        Some(_) => Err(Redirect::to("/ui/dashboard")),
        None => {
            let mut redis = state.redis.clone();
            let nonce = Nonce::generate_nonce(16);
            let _: RedisResult<()> = redis.set_ex(&nonce, &nonce, 600).await;
            Ok(LoginTemplate {
                cf_site_key: state.cf_site_key.to_string(),
                nonce,
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
    }
}
