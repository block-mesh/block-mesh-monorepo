use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn increment_usage(
    transaction: &mut Transaction<'_, Postgres>,
    usages_id: &Uuid,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
    sqlx::query!(
        r#"
        UPDATE usages SET usage = usage + 1, updated_at = $2 WHERE id = $1
        "#,
        usages_id,
        now,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
