use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::domain::aggregate::AggregateName;
use crate::ws::broadcaster::Broadcaster;
use crate::ws::cron_reports_controller::CronReportAggregateEntry;
use crate::ws::task_scheduler::TaskScheduler;
use anyhow::Context;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use sqlx::PgPool;
use std::fmt::Debug;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    pub broadcaster: Broadcaster,
    pub task_scheduler: TaskScheduler<WsServerMessage>,
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            broadcaster: Broadcaster::new(),
            task_scheduler: TaskScheduler::new(),
        }
    }
}

pub async fn settings_loop(
    pool: &PgPool,
    user_id: &Uuid,
    period: Duration,
    messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
    window_size: usize,
    broadcaster: Broadcaster,
) -> anyhow::Result<()> {
    let aggregate =
        get_or_create_aggregate_by_user_and_name_pool(pool, AggregateName::CronReports, user_id)
            .await?;
    let mut transaction = pool.begin().await?;
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
    transaction.commit().await?;
    loop {
        tracing::info!("Starting new loop");
        let settings = fetch_latest_cron_settings(pool, user_id).await?;
        let new_period = settings.period;
        let new_messages = settings.messages;
        let new_window_size = settings.window_size;
        let new_used_window_size = broadcaster
            .queue_multiple(new_messages.clone(), new_window_size)
            .await;
        let new_queue_size = broadcaster.queue.lock().unwrap().len();
        tracing::info!("size = {}", new_queue_size);
        let mut transaction = pool.begin().await?;
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
        transaction.commit().await?;
        tokio::time::sleep(new_period).await;
    }
}

pub async fn fetch_latest_cron_settings(
    pool: &PgPool,
    user_id: &Uuid,
) -> anyhow::Result<CronReportAggregateEntry> {
    let aggregate =
        get_or_create_aggregate_by_user_and_name_pool(pool, AggregateName::CronReports, user_id)
            .await?;
    if aggregate.value.is_null() {
        Ok(CronReportAggregateEntry::default())
    } else {
        let settings: CronReportAggregateEntry = serde_json::from_value(aggregate.value)?;
        Ok(settings)
    }
}
