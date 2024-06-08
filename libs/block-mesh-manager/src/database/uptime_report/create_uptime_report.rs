use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "create_uptime_report", skip(transaction), ret, err)]
pub(crate) async fn create_uptime_report(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let nonce = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO uptime_reports (id, created_at, nonce, user_id) VALUES ($1, $2, $3, $4)"#,
        id,
        now,
        nonce,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
