use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::domain::aggregate::AggregateName;
use crate::ws::broadcaster::Broadcaster;
use crate::ws::cron_reports_controller::CronReportAggregateEntry;
use crate::ws::task_scheduler::TaskScheduler;
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use sqlx::PgPool;
use std::fmt::Debug;
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
