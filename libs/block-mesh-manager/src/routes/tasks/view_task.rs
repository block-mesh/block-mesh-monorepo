use crate::database::task::get_task_by_id::get_task_by_user_id;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "tasks/view_task.html")]
struct ViewTaskTemplate {
    pub raw_html: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewTaskParams {
    pub id: Uuid,
}

#[tracing::instrument(name = "view_task", skip(auth, pool))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Query(query): Query<ViewTaskParams>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let _ = auth.user.ok_or(Error::UserNotFound).unwrap();
    let task = get_task_by_user_id(&mut transaction, &query.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;

    if task.is_none() {
        return Err(Error::TaskNotFound);
    }
    let task = task.unwrap();

    if task.response_raw.is_none() {
        return Err(Error::TaskResponseNotFound);
    }

    Ok(ViewTaskTemplate {
        raw_html: task.response_raw.unwrap(),
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
