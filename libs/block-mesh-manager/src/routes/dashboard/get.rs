use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use chrono::Utc;
use sqlx::PgPool;
use tracing::Level;

use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_LOGO, BLOCK_MESH_SUPPORT_CHAT,
    BLOCK_MESH_SUPPORT_EMAIL, BLOCK_MESH_TWITTER,
};

use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::invite_code::get_number_of_users_invited::get_number_of_users_invited;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::database::task::count_user_tasks_by_status::count_user_tasks_by_status;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
struct DashboardTemplate {
    pub email: String,
    pub email_status: String,
    pub overall_uptime: f64,
    pub overall_task_count: i64,
    pub invite_code: String,
    pub number_of_users_invited: i64,
    pub user_since: i64,
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
    pub points: f64,
}

#[tracing::instrument(name = "dashboard", skip(auth), level = "trace",  err(level = Level::TRACE))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let db_user = get_user_opt_by_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?
        .ok_or(Error::UserNotFound)?;
    let overall_task_count =
        count_user_tasks_by_status(&mut transaction, &user.id, TaskStatus::Completed).await?;
    // let rank =
    //     get_user_rank_by_task_status(&mut transaction, user.id, TaskStatus::Completed).await?;
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    let number_of_users_invited = get_number_of_users_invited(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    let uptime_aggregate = get_or_create_aggregate_by_user_and_name_no_transaction(
        &pool,
        AggregateName::Uptime,
        user.id,
    )
    .await
    .map_err(Error::from)?;
    let overall_uptime = uptime_aggregate.value.as_f64().unwrap_or_default();
    let user_since = (Utc::now() - db_user.created_at).num_days();
    transaction.commit().await.map_err(Error::from)?;

    let points =
        (overall_uptime / (24 * 60 * 60) as f64) * 100.0 + (overall_task_count as f64 * 10.0);

    let template = DashboardTemplate {
        points,
        overall_uptime,
        overall_task_count,
        invite_code: user_invite_code.invite_code,
        number_of_users_invited,
        user_since,
        email: db_user.email,
        email_status: (if db_user.verified_email {
            "Verified"
        } else {
            "Not Verified"
        })
        .to_string(),
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
    Ok(template)
}
