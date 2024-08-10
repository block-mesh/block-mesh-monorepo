use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::api_token::find_token::find_token;
use crate::database::daily_stat::create_daily_stat::create_daily_stat;
use crate::database::daily_stat::get_daily_stat_by_user_id_and_day::get_daily_stat_by_user_id_and_day;
use crate::database::daily_stat::increment_tasks_count::increment_tasks_count;
use crate::database::task::find_task_by_task_id_and_status::find_task_by_task_id_and_status;
use crate::database::task::finish_task::finish_task;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::domain::task::TaskStatus;
use crate::errors::error::Error;
use axum::extract::{Query, Request};
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{SubmitTaskRequest, SubmitTaskResponse};
use chrono::Utc;
use http::StatusCode;
use http_body_util::BodyExt;
use sqlx::PgPool;

#[tracing::instrument(name = "submit_task", skip(pool, request, query), level = "trace", ret)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<SubmitTaskRequest>,
    request: Request,
) -> Result<Json<SubmitTaskResponse>, Error> {
    let (_parts, body) = request.into_parts();

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
    let bytes = body
        .collect()
        .await
        .map_err(|_| Error::FailedReadingBody)?
        .to_bytes();
    let response_raw = String::from_utf8(bytes.to_vec()).unwrap_or_else(|_| String::from(""));

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
    let daily_stat_opt =
        get_daily_stat_by_user_id_and_day(&mut transaction, user.id, Utc::now().date_naive())
            .await?;
    let daily_stat_opt_id = match daily_stat_opt {
        Some(daily_stat) => daily_stat.id,
        None => create_daily_stat(&mut transaction, user.id).await?,
    };

    if query.response_code.unwrap_or(520) == 200 {
        let tasks = get_or_create_aggregate_by_user_and_name_no_transaction(
            &mut transaction,
            AggregateName::Tasks,
            user.id,
        )
        .await?;

        update_aggregate(
            &pool,
            tasks.id.unwrap_or_default(),
            &serde_json::Value::from(tasks.value.as_i64().unwrap_or_default() + 1),
        )
        .await
        .map_err(Error::from)?;
    }

    increment_tasks_count(&mut transaction, daily_stat_opt_id).await?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(SubmitTaskResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
