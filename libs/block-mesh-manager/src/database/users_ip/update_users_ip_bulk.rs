use crate::database::ip_address::get_or_create_ip_address::get_or_create_ip_address;
use crate::database::users_ip::get_or_create_users_ip::get_or_create_users_ip;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn update_users_ip_bulk(
    transaction: &mut Transaction<'_, Postgres>,
    calls: &mut HashMap<(Uuid, String), String>,
) -> anyhow::Result<()> {
    for pair in calls.iter() {
        let key = pair.0;
        let user_id = key.0;
        let ip = pair.1;
        if ip.is_empty() {
            continue;
        }
        let ip_address = get_or_create_ip_address(transaction, ip).await;
        if let Ok(ip_address) = ip_address {
            let _ = get_or_create_users_ip(transaction, &user_id, &ip_address.id).await;
        }
    }
    Ok(())
}
