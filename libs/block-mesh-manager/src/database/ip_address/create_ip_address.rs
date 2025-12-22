use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn create_ip_address(
    transaction: &mut Transaction<'_, Postgres>,
    ip: &str,
) -> anyhow::Result<Uuid> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
            INSERT INTO ip_addresses
            (id, ip, created_at, enriched)
            VALUES ($1, $2, $3, $4)
        "#,
        id,
        ip,
        now,
        false
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
