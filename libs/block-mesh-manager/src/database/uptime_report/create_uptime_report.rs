use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create_uptime_report(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    ip: &Option<String>,
) -> anyhow::Result<Uuid> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();
    let nonce = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO uptime_reports (id, created_at, nonce, user_id, ip) VALUES ($1, $2, $3, $4, $5)"#,
        id,
        now,
        nonce,
        user_id,
        match ip {
            None => "NULL",
            Some(ip) => ip
        }
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
