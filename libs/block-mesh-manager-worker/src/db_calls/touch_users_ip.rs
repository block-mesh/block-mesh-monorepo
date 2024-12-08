use crate::utils::id::Id;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn touch_users_ip(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    ip: &String,
) -> Result<(), anyhow::Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let ip_address_id = sqlx::query_as!(
        Id,
        r#"
                INSERT
                INTO ip_addresses
                (id, ip, created_at, enriched)
                VALUES
                ($1, $2, $3, $4)
                ON CONFLICT (ip) DO UPDATE SET updated_at = $3
                RETURNING
                id
                "#,
        id,
        ip,
        now,
        false
    )
    .fetch_one(&mut **transaction)
    .await;

    if let Ok(ip_address_id) = ip_address_id {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query!(
            r#"
                    INSERT INTO users_ip
                    (id, user_id, ip_id, created_at, updated_at)
                    VALUES
                    ($1, $2, $3, $4, $5)
                    ON CONFLICT (user_id, ip_id) DO UPDATE SET updated_at = $5
                    RETURNING id
                    "#,
            id,
            user_id,
            ip_address_id.id,
            now.clone(),
            now
        )
        .fetch_one(&mut **transaction)
        .await?;
    }
    Ok(())
}
