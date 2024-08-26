use block_mesh_common::interfaces::server_api::UserIpInfo;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_user_ips", skip(transaction), ret, err, level = "trace")]
pub(crate) async fn get_user_ips(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<UserIpInfo>> {
    let ips = sqlx::query!(
        r#"
        SELECT
        ip_addresses.ip, ip_addresses.country, users_ip.updated_at
        FROM users_ip
        JOIN ip_addresses ON users_ip.ip_id = ip_addresses.id
        WHERE users_ip.user_id = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?
    .iter()
    .map(|row| UserIpInfo {
        updated_at: row.updated_at.clone(),
        ip: row.ip.clone(),
        country: row.country.clone(),
    })
    .collect();
    Ok(ips)
}
