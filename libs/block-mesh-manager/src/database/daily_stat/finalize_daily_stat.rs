use crate::domain::daily_stat::DailyStatStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "finalize_daily_stat", skip(transaction), ret, err)]
pub(crate) async fn finalize_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE daily_stats SET status = $1 WHERE id = $2"#,
        DailyStatStatus::Finalized.to_string(),
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
