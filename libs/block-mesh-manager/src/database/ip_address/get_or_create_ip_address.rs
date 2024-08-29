use crate::domain::ip_address::IpAddress;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_or_create_ip_address(
    transaction: &mut Transaction<'_, Postgres>,
    ip: &str,
) -> anyhow::Result<IpAddress> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let out = sqlx::query_as!(
        IpAddress,
        r#"
            INSERT
            INTO ip_addresses
            (id, ip, created_at, enriched)
            VALUES
            ($1, $2, $3, $4)
            ON CONFLICT (ip) DO UPDATE SET updated_at = $3
            RETURNING
            id,
            ip,
            created_at,
            updated_at,
            latitude,
            longitude,
            country,
            city,
            region,
            timezone,
            isp,
            enriched
        "#,
        id,
        ip,
        now,
        false
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(out)
}
