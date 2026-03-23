use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_wallet_connected_sent(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    wallet_connected_sent: bool,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE users
        SET wallet_connected_sent = $1
        WHERE id = $2
          AND wallet_connected_sent IS DISTINCT FROM $1
        RETURNING id
        "#,
        wallet_connected_sent,
        user_id
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(result.is_some())
}
