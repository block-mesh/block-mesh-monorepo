use block_mesh_common::interfaces::server_api::UserIpInfo;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_user_ips(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    limit: i64,
) -> anyhow::Result<Vec<UserIpInfo>> {
    let ips = sqlx::query!(
        /*
        SELECT
        ip_addresses.ip, ip_addresses.country, users_ip.updated_at
        FROM users_ip
        JOIN ip_addresses ON users_ip.ip_id = ip_addresses.id
        WHERE users_ip.user_id = $1
        ORDER BY users_ip.updated_at DESC
        LIMIT $2
         */
        r#"
        WITH user_ip_data (ip_id, updated_at) AS (
           SELECT ip_id, updated_at
           FROM users_ip
           WHERE user_id = $1
           ORDER BY updated_at DESC
           LIMIT $2
        )
        SELECT ip_addresses.ip, ip_addresses.country, user_ip_data.updated_at
        FROM ip_addresses
        JOIN user_ip_data ON ip_addresses.id = user_ip_data.ip_id
        "#,
        user_id,
        limit
    )
    .fetch_all(&mut **transaction)
    .await?
    .iter()
    .map(|row| UserIpInfo {
        updated_at: row.updated_at,
        ip: row.ip.clone(),
        country: row.country.clone(),
    })
    .collect();
    Ok(ips)
}
