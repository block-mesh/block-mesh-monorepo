use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::database::api_token::find_token::find_token;
use crate::database::daily_stat::get_or_create_daily_stat::get_or_create_daily_stat;
use crate::database::daily_stat::increment_tasks_count::increment_tasks_count;
use crate::database::task::find_task_by_task_id_and_status::find_task_by_task_id_and_status;
use crate::database::task::finish_task::finish_task;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use axum::extract::{Query, Request, State};
use axum::Json;
use block_mesh_common::interfaces::db_messages::{AggregateMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::{
    HandlerMode, SubmitTaskRequest, SubmitTaskResponse,
};
use http::StatusCode;
use http_body_util::BodyExt;
use std::sync::Arc;

pub async fn submit_task_content(
    state: Arc<AppState>,
    query: SubmitTaskRequest,
    request: Option<Request>,
    mode: HandlerMode,
) -> Result<Json<SubmitTaskResponse>, Error> {
    let pool = state.pool.clone();

    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if user.email.to_ascii_lowercase() != query.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }
    let task =
        find_task_by_task_id_and_status(&mut transaction, &query.task_id, TaskStatus::Assigned)
            .await?
            .ok_or(Error::TaskNotFound)?;
    if task.assigned_user_id.is_some() && task.assigned_user_id.unwrap() != user.id {
        return Err(Error::TaskAssignedToAnotherUser);
    }

    let response_raw = match mode {
        HandlerMode::Http => match request {
            Some(request) => {
                let (_parts, body) = request.into_parts();
                let bytes = body
                    .collect()
                    .await
                    .map_err(|_| Error::FailedReadingBody)?
                    .to_bytes();
                String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""))
            }
            None => {
                return Err(Error::InternalServer);
            }
        },
        HandlerMode::WebSocket => match query.response_body {
            Some(body) => body,
            None => {
                return Err(Error::InternalServer);
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
    let daily_stat = get_or_create_daily_stat(&mut transaction, &user.id).await?;
    increment_tasks_count(&mut transaction, daily_stat.id).await?;
    transaction.commit().await.map_err(Error::from)?;

    if query.response_code.unwrap_or(520) == 200 {
        let mut transaction = pool.begin().await.map_err(Error::from)?;
        let tasks = get_or_create_aggregate_by_user_and_name(
            &mut transaction,
            AggregateName::Tasks,
            &user.id,
        )
        .await?;
        transaction.commit().await.map_err(Error::from)?;
        let _ = state
            .tx_aggregate_agg
            .send_async(AggregateMessage {
                msg_type: DBMessageTypes::AggregateMessage,
                id: tasks.id,
                value: serde_json::Value::from(tasks.value.as_i64().unwrap_or_default() + 1),
            })
            .await;
    }

    Ok(Json(SubmitTaskResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}

#[tracing::instrument(name = "submit_task", skip_all, level = "trace", ret)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SubmitTaskRequest>,
    request: Request,
) -> Result<Json<SubmitTaskResponse>, Error> {
    submit_task_content(state, query, Some(request), HandlerMode::Http).await
}
