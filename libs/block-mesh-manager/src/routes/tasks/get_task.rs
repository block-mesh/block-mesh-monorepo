use crate::database::api_token::find_token::find_token_pool;
use crate::database::daily_stat::create_daily_stat::create_daily_stat;
use crate::database::daily_stat::get_daily_stat_by_user_id_and_day::get_daily_stat_by_user_id_and_day;
use crate::database::task::find_task_assigned_to_user::find_task_assigned_to_user;
use crate::database::task::find_task_by_status::find_task_by_status;
use crate::database::task::update_task_assigned::update_task_assigned;
use crate::database::user::get_user_by_id::get_user_opt_by_id_pool;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use crate::middlewares::rate_limit::filter_request;
use crate::startup::application::AppState;
use anyhow::Context;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetTaskRequest, GetTaskResponse};
use chrono::Utc;
use http::HeaderMap;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "get_task", skip(body, pool, headers, state), fields(email = body.email), level = "trace")]
pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetTaskRequest>,
) -> Result<Json<Option<GetTaskResponse>>, Error> {
    let api_token = find_token_pool(&pool, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id_pool(&pool, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let ip = headers
        .get("cf-connecting-ip")
        .context("Missing CF-CONNECTING-IP")?
        .to_str()
        .context("Unable to STR CF-CONNECTING-IP")?;

    let filter = filter_request(&mut redis, &user.id, ip).await;
    if filter.is_err() {
        return Ok(Json(None));
    }
    if !filter.unwrap() {
        return Ok(Json(None));
    }

    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let task = find_task_assigned_to_user(&mut transaction, &user.id).await?;
    if let Some(task) = task {
        return Ok(Json(Some(GetTaskResponse {
            id: task.id,
            url: task.url,
            method: task.method.to_string(),
            headers: task.headers,
            body: task.body,
        })));
    }
    let task = find_task_by_status(&mut transaction, TaskStatus::Pending).await?;
    let task = match task {
        Some(v) => v,
        None => return Ok(Json(None)),
    };
    let daily_stat_opt =
        get_daily_stat_by_user_id_and_day(&mut transaction, user.id, Utc::now().date_naive())
            .await?;
    if daily_stat_opt.is_none() {
        create_daily_stat(&mut transaction, user.id).await?;
    }
    update_task_assigned(&mut transaction, task.id, user.id, TaskStatus::Assigned).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(Some(GetTaskResponse {
        id: task.id,
        url: task.url,
        method: task.method.to_string(),
        headers: task.headers,
        body: task.body,
    })))
}
