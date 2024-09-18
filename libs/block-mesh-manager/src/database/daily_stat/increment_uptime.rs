use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn increment_uptime(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    uptime: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE daily_stats SET uptime = uptime + $1 WHERE id = $2"#,
        uptime,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
