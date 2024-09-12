use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::domain::aggregate::AggregateName;
use crate::ws::broadcaster::Broadcaster;
use crate::ws::cron_reports_controller::{
    CronReportSettings, CronReportStats, CronReportsController,
};
use crate::ws::task_scheduler::TaskScheduler;
use anyhow::Context;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use sqlx::PgPool;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::watch::Sender;
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

    pub async fn cron_reports(
        &mut self,
        period: Duration,
        messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
        window_size: usize,
        pool: PgPool,
    ) -> anyhow::Result<CronReportsController> {
        self.broadcaster
            .cron_reports(period, messages, window_size, pool)
            .await
    }
}

pub async fn settings_loop(
    pool: &PgPool,
    user_id: &Uuid,
    period: Duration,
    messages: impl Into<Vec<WsServerMessage>> + Clone + Send + 'static,
    window_size: usize,
    broadcaster: Broadcaster,
    stats_tx: Sender<CronReportStats>,
) -> anyhow::Result<()> {
    let aggregate =
        get_or_create_aggregate_by_user_and_name_pool(pool, AggregateName::CronReports, user_id)
            .await?;
    let mut transaction = pool.begin().await?;
    update_aggregate(
        &mut transaction,
        &aggregate.id,
        &serde_json::to_value(CronReportSettings::new(
            Some(period),
            Some(messages.clone().into()),
            Some(window_size),
        ))
        .context("Failed to parse cron report settings")?,
    )
    .await?;
    let messages = messages.into();
    loop {
        let messages = messages.clone();
        let settings = fetch_latest_cron_settings(pool, user_id).await?;
        let new_period = settings.period.unwrap_or(period);
        let new_messages = settings.messages.unwrap_or(messages);
        let new_window_size = settings.window_size.unwrap_or(window_size);
        let sent_messages_count = broadcaster
            .queue_multiple(new_messages.clone(), window_size)
            .await;
        let queue_size = broadcaster.queue.lock().unwrap().len();
        if let Err(error) = stats_tx.send(CronReportStats::new(
            new_messages,
            new_window_size,
            sent_messages_count,
            queue_size,
            new_period,
        )) {
            // TODO (send_if_modified, send_modify, or send_replace) can be used instead
            tracing::error!("Could not sent stats, no watchers: {error}");
        }
        tokio::time::sleep(new_period).await;
    }
}

async fn fetch_latest_cron_settings(
    pool: &PgPool,
    user_id: &Uuid,
) -> anyhow::Result<CronReportSettings> {
    let aggregate =
        get_or_create_aggregate_by_user_and_name_pool(pool, AggregateName::CronReports, user_id)
            .await?;
    if aggregate.value.is_null() {
        Ok(CronReportSettings::default())
    } else {
        let settings: CronReportSettings = serde_json::from_value(aggregate.value)?;
        Ok(settings)
    }
}
