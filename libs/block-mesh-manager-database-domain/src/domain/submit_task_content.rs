use crate::domain::aggregate::AggregateName;
use crate::domain::create_daily_stat::get_or_create_daily_stat;
use crate::domain::find_task_by_task_id_and_status::find_task_by_task_id_and_status;
use crate::domain::finish_task::finish_task;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::domain::get_user_and_api_token::get_user_and_api_token_by_email;
use crate::domain::increment_tasks_count::increment_tasks_count;
use crate::domain::notify_worker::notify_worker;
use crate::domain::task::TaskStatus;
use anyhow::{anyhow, Error};
use axum::extract::Request;
use axum::Json;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::{
    HandlerMode, SubmitTaskRequest, SubmitTaskResponse,
};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use http_body_util::BodyExt;
use sqlx::PgPool;
use std::env;
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
    follower_pool: &PgPool,
    channel_pool: &PgPool,
    query: SubmitTaskRequest,
    request: Option<Request>,
    mode: HandlerMode,
) -> Result<Json<SubmitTaskResponse>, Error> {
    let mut follower_transaction = create_txn(follower_pool).await?;

    let user = get_user_and_api_token_by_email(&mut follower_transaction, &query.email)
        .await?
        .ok_or_else(|| anyhow!("User Not Found"))?;
    if user.token.as_ref() != &query.api_token {
        commit_txn(follower_transaction).await?;
        return Err(anyhow!("Api Token Mismatch"));
    }
    let task = find_task_by_task_id_and_status(
        &mut follower_transaction,
        &query.task_id,
        TaskStatus::Assigned,
    )
    .await?
    .ok_or(anyhow!("Task Not Found".to_string()))?;
    if task.assigned_user_id.is_some() && task.assigned_user_id.unwrap() != user.user_id {
        commit_txn(follower_transaction).await?;
        return Err(anyhow!("Task Assigned To Another User".to_string(),));
    }

    let response_raw = match mode {
        HandlerMode::Http => match request {
            Some(request) => extract_body(request).await?,
            None => {
                commit_txn(follower_transaction).await?;
                return Err(anyhow!("Internal Server Error".to_string()));
            }
        },
        HandlerMode::WebSocket => match query.response_body {
            Some(body) => body,
            None => {
                commit_txn(follower_transaction).await?;
                return Err(anyhow!("Internal Server Error".to_string()));
            }
        },
    };
    commit_txn(follower_transaction).await?;
    let mut transaction = create_txn(pool).await?;
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
    let daily_stat = get_or_create_daily_stat(&mut transaction, &user.user_id, None).await?;
    let task_bonus = env::var("TASK_BONUS")
        .unwrap_or("0".to_string())
        .parse()
        .unwrap_or(0);
    increment_tasks_count(&mut transaction, daily_stat.id, 1 + task_bonus).await?;
    commit_txn(transaction).await?;

    if query.response_code.unwrap_or(520) == 200 {
        let mut transaction = create_txn(pool).await?;
        let tasks = get_or_create_aggregate_by_user_and_name(
            &mut transaction,
            AggregateName::Tasks,
            &user.user_id,
        )
        .await?;
        commit_txn(transaction).await?;
        let _ = notify_worker(
            channel_pool,
            &[DBMessage::AggregateMessage(AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: tasks.id,
                value: serde_json::Value::from(tasks.value.as_i64().unwrap_or_default() + 1),
            })],
        )
        .await;
    }
    Ok(Json(SubmitTaskResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
