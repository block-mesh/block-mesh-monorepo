use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_or_create_users_ip(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    ip_id: &Uuid,
) -> anyhow::Result<Uuid> {
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
        ip_id,
        now.clone(),
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(id)
}
