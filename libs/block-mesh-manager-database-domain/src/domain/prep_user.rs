use crate::domain::aggregate::AggregateName;
use crate::domain::create_daily_stat::create_daily_stat;
use crate::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "prep_user", skip_all)]
pub async fn prep_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    let _ = get_or_create_aggregate_by_user_and_name(transaction, AggregateName::Tasks, user_id)
        .await?;
    let _ = get_or_create_aggregate_by_user_and_name(transaction, AggregateName::Uptime, user_id)
        .await?;
    let _ = get_or_create_aggregate_by_user_and_name(transaction, AggregateName::Download, user_id)
        .await?;
    let _ = get_or_create_aggregate_by_user_and_name(transaction, AggregateName::Latency, user_id)
        .await?;
    let _ = create_daily_stat(transaction, user_id).await;
    Ok(())
}
