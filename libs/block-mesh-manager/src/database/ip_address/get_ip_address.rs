use crate::domain::ip_address::IpAddress;
use sqlx::{Postgres, Transaction};

pub async fn get_ip_address(
    transaction: &mut Transaction<'_, Postgres>,
    ip: &str,
) -> anyhow::Result<IpAddress> {
    Ok(sqlx::query_as!(
        IpAddress,
        r#"
            SELECT
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
            FROM ip_addresses
            WHERE ip = $1
            LIMIT 1
        "#,
        ip
    )
    .fetch_one(&mut **transaction)
    .await?)
}
