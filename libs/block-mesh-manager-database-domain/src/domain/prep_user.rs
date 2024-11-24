use crate::domain::bulk_get_or_create_aggregate_by_user_and_name::bulk_get_or_create_aggregate_by_user_and_name;
use crate::domain::create_daily_stat::get_or_create_daily_stat;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "prep_user", skip_all)]
pub async fn prep_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    let _ = bulk_get_or_create_aggregate_by_user_and_name(transaction, user_id).await?;
    let _ = get_or_create_daily_stat(transaction, user_id, None).await;
    Ok(())
}
