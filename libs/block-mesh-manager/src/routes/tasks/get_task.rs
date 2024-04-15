use crate::database::api_token::find_token::find_token;
use crate::database::daily_stat::create_daily_stat::create_daily_stat;
use crate::database::daily_stat::get_daily_stat_by_user_id_and_day::get_daily_stat_by_user_id_and_day;
use crate::database::task::find_task_by_excluded_user_id_and_status::find_task_by_excluded_user_id_and_status;
use crate::database::task::update_task_assigned::update_task_assigned;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interface::{GetTaskRequest, GetTaskResponse};
use chrono::Utc;
use sqlx::PgPool;

#[tracing::instrument(name = "get_task", skip(body, pool))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetTaskRequest>,
) -> Result<Json<GetTaskResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email != body.email {
        return Err(Error::UserNotFound);
    }
    let task =
        find_task_by_excluded_user_id_and_status(&mut transaction, &user.id, TaskStatus::Pending)
            .await?
            .ok_or(Error::TaskNotFound)?;
    let daily_stat_opt =
        get_daily_stat_by_user_id_and_day(&mut transaction, user.id, Utc::now().date_naive())
            .await?;
    if daily_stat_opt.is_none() {
        create_daily_stat(&mut transaction, user.id).await?;
    }
    update_task_assigned(&mut transaction, task.id, user.id, TaskStatus::Assigned).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetTaskResponse {
        id: task.id,
        url: task.url,
        method: task.method.to_string(),
        headers: task.headers,
        body: task.body,
    }))
}
