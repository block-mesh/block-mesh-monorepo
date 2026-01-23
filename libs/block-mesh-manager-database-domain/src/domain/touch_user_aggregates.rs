use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

#[tracing::instrument(name = "touch_user_aggregates", skip_all)]
pub async fn touch_user_aggregates(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
    sqlx::query!(
        r#"UPDATE aggregates SET updated_at = $1 WHERE user_id = $2"#,
        now,
        user_id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
