use crate::domain::aggregate::AggregateName;
use crate::domain::create_daily_stat::create_daily_stat;
use crate::domain::find_task_by_task_id_and_status::find_task_by_task_id_and_status;
use crate::domain::find_token::find_token;
use crate::domain::finish_task::finish_task;
use crate::domain::get_daily_stat_of_user::get_daily_stat_of_user;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::domain::get_user_opt_by_id::get_user_opt_by_id;
use crate::domain::increment_tasks_count::increment_tasks_count;
use crate::domain::notify_worker::notify_worker;
use crate::domain::task::TaskStatus;
use anyhow::{anyhow, Error};
use axum::extract::Request;
use axum::Json;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::{
    HandlerMode, SubmitTaskRequest, SubmitTaskResponse,
};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use http_body_util::BodyExt;
use sqlx::PgPool;
use tracing::{span, Level};

#[tracing::instrument(name = "extract_body", skip_all)]
pub async fn extract_body(request: Request) -> anyhow::Result<String> {
    let (_parts, body) = request.into_parts();
    let bytes = body
        .collect()
        .await
        .map_err(|e| anyhow!(e.to_string()))?
        .to_bytes();
    let span = span!(Level::INFO, "bytes", len = bytes.len()).entered();
    span.exit();
    Ok(String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from("")))
}

#[tracing::instrument(name = "submit_task_content", skip_all)]
pub async fn submit_task_content(
    pool: &PgPool,
    query: SubmitTaskRequest,
    request: Option<Request>,
    mode: HandlerMode,
) -> Result<Json<SubmitTaskResponse>, Error> {
    let mut transaction = create_txn(pool).await?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(anyhow!("Api Token Not Found".to_string()))?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| anyhow!("User Not Found".to_string()))?;
    if user.email.to_ascii_lowercase() != query.email.to_ascii_lowercase() {
        commit_txn(transaction).await?;
        return Err(anyhow!("User Not Found".to_string()));
    }
    let task =
        find_task_by_task_id_and_status(&mut transaction, &query.task_id, TaskStatus::Assigned)
            .await?
            .ok_or(anyhow!("Token Not Found".to_string()))?;
    if task.assigned_user_id.is_some() && task.assigned_user_id.unwrap() != user.id {
        commit_txn(transaction).await?;
        return Err(anyhow!("Task Assigned To Another User".to_string(),));
    }

    let response_raw = match mode {
        HandlerMode::Http => match request {
            Some(request) => extract_body(request).await?,
            None => {
                commit_txn(transaction).await?;
                return Err(anyhow!("Internal Server Error".to_string()));
            }
        },
        HandlerMode::WebSocket => match query.response_body {
            Some(body) => body,
            None => {
                commit_txn(transaction).await?;
                return Err(anyhow!("Internal Server Error".to_string()));
            }
        },
    };

    finish_task(
        &mut transaction,
        query.task_id,
        query.response_code,
        Option::from(response_raw),
        match query.response_code.unwrap_or(520) {
            520 => TaskStatus::Failed,
            _ => TaskStatus::Completed,
        },
        &query.country.unwrap_or_default(),
        &query.ip.unwrap_or_default(),
        &query.asn.unwrap_or_default(),
        &query.colo.unwrap_or_default(),
        query.response_time.unwrap_or_default(),
    )
    .await?;
    let _ = create_daily_stat(&mut transaction, user.id).await;
    let daily_stat = get_daily_stat_of_user(&mut transaction, user.id).await?;
    increment_tasks_count(&mut transaction, daily_stat.id).await?;
    commit_txn(transaction).await?;

    if query.response_code.unwrap_or(520) == 200 {
        let mut transaction = create_txn(pool).await?;
        let tasks = get_or_create_aggregate_by_user_and_name(
            &mut transaction,
            AggregateName::Tasks,
            &user.id,
        )
        .await?;
        commit_txn(transaction).await?;
        let _ = notify_worker(
            pool,
            AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: tasks.id,
                value: serde_json::Value::from(tasks.value.as_i64().unwrap_or_default() + 1),
            },
        )
        .await;
    }
    Ok(Json(SubmitTaskResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
