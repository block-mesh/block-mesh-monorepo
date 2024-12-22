use crate::database::task::find_task_assigned_to_user::find_task_assigned_to_user;
use crate::database::task::find_task_by_status::find_task_by_status;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::Context;
use axum::extract::State;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetTaskRequest, GetTaskResponse};
use block_mesh_manager_database_domain::domain::create_daily_stat::get_or_create_daily_stat;
use block_mesh_manager_database_domain::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use block_mesh_manager_database_domain::domain::task::TaskStatus;
use block_mesh_manager_database_domain::domain::update_task_assigned::update_task_assigned;
use chrono::{Duration, Utc};
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
    if state.rate_limit {
        let expiry = get_envar("FILTER_REQUEST_EXPIRY_SECONDS")
            .await
            .parse()
            .unwrap_or(3u64);
        let date = Utc::now() + Duration::milliseconds(expiry as i64);
        let key = format!("get_task_{}", header_ip);
        if state.rate_limiter.get(&key).is_some() {
            return Ok(Json(None));
        }
        state.rate_limiter.insert(key, Some(date));
        let key = format!("get_task_{}", body.api_token);
        if state.rate_limiter.get(&key).is_some() {
            return Ok(Json(None));
        }
        state.rate_limiter.insert(key, Some(date));
    }
    let mut follower_transaction = create_txn(&state.follower_pool).await?;
    let user = get_user_and_api_token_by_email(&mut follower_transaction, &body.email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.token.as_ref() != &body.api_token {
        commit_txn(follower_transaction).await?;
        return Err(Error::ApiTokenNotFound);
    }
    let task = find_task_assigned_to_user(&mut follower_transaction, &user.user_id).await?;
    if let Some(task) = task {
        return Ok(Json(Some(GetTaskResponse {
            id: task.id,
            url: task.url,
            method: task.method.to_string(),
            headers: task.headers,
            body: task.body,
        })));
    }
    let task = find_task_by_status(&mut follower_transaction, TaskStatus::Pending).await?;
    let task = match task {
        Some(v) => v,
        None => return Ok(Json(None)),
    };
    commit_txn(follower_transaction).await?;
    let mut transaction = create_txn(&pool).await?;
    let _ = get_or_create_daily_stat(&mut transaction, &user.user_id, None).await?;
    update_task_assigned(
        &mut transaction,
        task.id,
        user.user_id,
        TaskStatus::Assigned,
    )
    .await?;
    commit_txn(transaction).await?;
    Ok(Json(Some(GetTaskResponse {
        id: task.id,
        url: task.url,
        method: task.method.to_string(),
        headers: task.headers,
        body: task.body,
    })))
}
