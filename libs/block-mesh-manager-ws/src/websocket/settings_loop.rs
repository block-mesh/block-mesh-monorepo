use crate::websocket::manager::broadcaster::Broadcaster;
use anyhow::Context;
use block_mesh_common::interfaces::server_api::CronReportAggregateEntry;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::fetch_latest_cron_settings::fetch_latest_cron_settings;
use block_mesh_manager_database_domain::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use block_mesh_manager_database_domain::domain::update_aggregate::update_aggregate;
use block_mesh_manager_database_domain::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

#[tracing::instrument(name = "settings_loop", skip_all)]
pub async fn settings_loop(
    pool: PgPool,
    server_user_id: Uuid,
    period: Duration,
    window_size: usize,
    broadcaster: Broadcaster,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(&pool).await?;
    let aggregate = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::CronReports,
        &server_user_id,
    )
    .await?;
    update_aggregate(
        &mut transaction,
        &aggregate.id,
        &serde_json::to_value(CronReportAggregateEntry::new(
            period,
            vec![],
            window_size,
            0,
            0,
        ))
        .context("Failed to parse cron report settings")?,
    )
    .await?;
    commit_txn(transaction).await?;
    loop {
        let settings = fetch_latest_cron_settings(&pool, &server_user_id).await?;
        let new_period = settings.period;
        let new_messages = settings.messages;
        let new_window_size = settings.window_size;
        let queued = broadcaster
            .queue_multiple(new_messages.clone(), new_window_size)
            .await;
        let new_used_window_size = queued.len();
        let new_queue_size = broadcaster.queue.lock().await.len();
        let mut transaction = create_txn(&pool).await?;
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
