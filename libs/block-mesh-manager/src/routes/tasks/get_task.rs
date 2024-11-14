use crate::database::task::find_task_assigned_to_user::find_task_assigned_to_user;
use crate::database::task::find_task_by_status::find_task_by_status;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::middlewares::rate_limit::filter_request;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetTaskRequest, GetTaskResponse};
use block_mesh_manager_database_domain::domain::create_daily_stat::create_daily_stat;
use block_mesh_manager_database_domain::domain::find_token::find_token;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use block_mesh_manager_database_domain::domain::task::TaskStatus;
use block_mesh_manager_database_domain::domain::task_limit::TaskLimit;
use block_mesh_manager_database_domain::domain::update_task_assigned::update_task_assigned;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::HeaderMap;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "get_task", skip_all)]
pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetTaskRequest>,
) -> Result<Json<Option<GetTaskResponse>>, Error> {
    let limit = get_envar("TASK_LIMIT").await.parse().unwrap_or(10);
    let mut redis = state.redis.clone();
    let mut redis_user = match TaskLimit::get_task_limit(&body.api_token, &mut redis, limit).await {
        Ok(r) => r,
        Err(_) => return Err(Error::TaskLimit),
    };

    let app_env = get_envar("APP_ENVIRONMENT").await;
    let header_ip = if app_env != "local" {
        headers
            .get("cf-connecting-ip")
            .context("Missing CF-CONNECTING-IP")?
            .to_str()
            .context("Unable to STR CF-CONNECTING-IP")?
    } else {
        "127.0.0.1"
    };

    let filter = filter_request(&mut redis, &body.api_token, header_ip).await;
    if filter.is_err() || !filter? {
        return Err(Error::NotAllowedRateLimit);
    }

    let mut transaction = create_txn(&pool).await?;
    let api_token = find_token(&mut transaction, &body.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;

    if user.email.to_ascii_lowercase() != body.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }

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
    let _ = create_daily_stat(&mut transaction, &user.id).await?;
    update_task_assigned(&mut transaction, task.id, user.id, TaskStatus::Assigned).await?;
    commit_txn(transaction).await?;
    let task_bonus = get_envar("TASK_BONUS").await.parse().unwrap_or(0);
    let expire = 10u64 * Backend::get_expire().await as u64;
    redis_user.tasks += 1 + task_bonus;
    TaskLimit::save_user(&mut redis, &redis_user, expire).await;
    Ok(Json(Some(GetTaskResponse {
        id: task.id,
        url: task.url,
        method: task.method.to_string(),
        headers: task.headers,
        body: task.body,
    })))
}
