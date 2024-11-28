use crate::database::task::get_tasks_rpc_results::{get_tasks_rpc_results, RpcResults};
use crate::errors::error::Error;
use crate::startup::application::AppState;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::Extension;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use sqlx::PgPool;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "rpc_dashboard/rpc_table.html")]
struct RpcDashboardTemplate {
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
    pub results: Vec<RpcResults>,
    pub cf_site_key: String,
}

#[tracing::instrument(name = "rpc_dashboard", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let results = get_tasks_rpc_results(&mut transaction, 600).await?;
    transaction.commit().await.map_err(Error::from)?;
    let template = RpcDashboardTemplate {
        cf_site_key: state.cf_site_key.to_string(),
        chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
        app_server: BLOCK_MESH_APP_SERVER.to_string(),
        github: BLOCK_MESH_GITHUB.to_string(),
        twitter: BLOCK_MESH_TWITTER.to_string(),
        gitbook: BLOCK_MESH_GITBOOK.to_string(),
        logo: BLOCK_MESH_LOGO.to_string(),
        image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
        support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
        chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
        results,
    };
    Ok(template)
}
