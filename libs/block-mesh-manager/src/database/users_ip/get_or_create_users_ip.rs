use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "get_or_create_users_ip",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn get_or_create_users_ip(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    ip_id: &Uuid,
) -> anyhow::Result<Uuid> {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        WITH
            extant AS (
                SELECT id FROM users_ip WHERE user_id = $2 AND ip_id = $3
            ),
            inserted AS (
                INSERT INTO users_ip (id, user_id, ip_id)
                SELECT $1, $2, $3
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id
            )
        SELECT id FROM inserted
        UNION ALL
        SELECT id FROM extant
        "#,
        id,
        user_id,
        ip_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(id)
}
