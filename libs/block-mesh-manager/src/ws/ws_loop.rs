use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::task::find_users_tasks::find_users_tasks;
use crate::database::task::update_task_assigned::update_task_assigned;
use crate::domain::aggregate::AggregateName;
use crate::domain::task::TaskStatus;
use crate::utils::instrument_wrapper::{commit_txn, create_txn};
use crate::ws::broadcaster::Broadcaster;
use crate::ws::connection_manager::fetch_latest_cron_settings;
use crate::ws::cron_reports_controller::CronReportAggregateEntry;
use anyhow::Context;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "ws_loop", skip_all)]
pub async fn ws_loop(
    pool: &PgPool,
    user_id: &Uuid,
    period: Duration,
    messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
    window_size: usize,
    broadcaster: Broadcaster,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(pool).await?;
    let aggregate = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::CronReports,
        user_id,
    )
    .await?;
    update_aggregate(
        &mut transaction,
        &aggregate.id,
        &serde_json::to_value(CronReportAggregateEntry::new(
            period,
            messages.clone().into(),
            window_size,
            0,
            0,
        ))
        .context("Failed to parse cron report settings")?,
    )
    .await?;
    commit_txn(transaction).await?;
    loop {
        let settings = fetch_latest_cron_settings(pool, user_id).await?;
        let new_period = settings.period;
        let new_messages = settings.messages;
        let new_window_size = settings.window_size;
        let mut queued = broadcaster
            .queue_multiple(new_messages.clone(), new_window_size)
            .await;
        let new_used_window_size = queued.len();
        let new_queue_size = broadcaster.queue.lock().unwrap().len();

        let mut transaction = create_txn(pool).await?;
        let mut tasks = find_users_tasks(&mut transaction, new_window_size as i64).await?;
        loop {
            let task = match tasks.pop() {
                Some(t) => t,
                None => break,
            };
            let queue = match queued.pop() {
                Some(q) => q,
                None => break,
            };
            let user_id = queue.0;
            let _ = broadcaster
                .broadcast_to_user(
                    vec![WsServerMessage::AssignTask(GetTaskResponse {
                        id: task.id,
                        url: task.url,
                        method: task.method.to_string(),
                        headers: task.headers,
                        body: task.body,
                    })],
                    queue,
                )
                .await;
            update_task_assigned(&mut transaction, task.id, user_id, TaskStatus::Assigned).await?;
        }
        update_aggregate(
            &mut transaction,
            &aggregate.id,
            &serde_json::to_value(CronReportAggregateEntry::new(
                new_period,
                new_messages,
                new_window_size,
                new_used_window_size,
                new_queue_size,
            ))
            .context("Failed to parse cron report settings")?,
        )
        .await?;
        commit_txn(transaction).await?;
        tokio::time::sleep(new_period).await;
    }
}
