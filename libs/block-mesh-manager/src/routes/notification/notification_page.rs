use crate::errors::error::Error;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationQueryParams {
    pub summary: String,
    pub detailed: String,
    pub go_to: String,
}

#[derive(Template, Debug, Serialize, Deserialize)]
#[template(path = "notification_page.html")]
pub struct NotificationTemplate {
    pub summary: String,
    pub detailed: String,
    pub go_to: String,
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

#[tracing::instrument(name = "notification_page")]
pub async fn handler(error: Query<NotificationQueryParams>) -> Result<impl IntoResponse, Error> {
    let notification_template = NotificationTemplate {
        summary: error.summary.clone(),
        detailed: error.detailed.clone(),
        go_to: error.go_to.clone(),
        chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
        app_server: BLOCK_MESH_APP_SERVER.to_string(),
        github: BLOCK_MESH_GITHUB.to_string(),
        twitter: BLOCK_MESH_TWITTER.to_string(),
        gitbook: BLOCK_MESH_GITBOOK.to_string(),
        logo: BLOCK_MESH_LOGO.to_string(),
        image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
        support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
        chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
    };
    Ok(notification_template)
}
