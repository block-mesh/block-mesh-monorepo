use crate::domain::aggregate::AggregateName;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use crate::utils::instrument_wrapper::{commit_txn, create_txn};
use block_mesh_common::interfaces::server_api::CronReportAggregateEntry;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "fetch_latest_cron_settings", skip_all)]
pub async fn fetch_latest_cron_settings(
    pool: &PgPool,
    user_id: &Uuid,
) -> anyhow::Result<CronReportAggregateEntry> {
    let mut transaction = create_txn(pool).await?;
    let aggregate = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::CronReports,
        user_id,
    )
    .await?;
    commit_txn(transaction).await?;
    if aggregate.value.is_null() {
        Ok(CronReportAggregateEntry::default())
    } else {
        let settings: CronReportAggregateEntry = serde_json::from_value(aggregate.value)?;
        Ok(settings)
    }
}
