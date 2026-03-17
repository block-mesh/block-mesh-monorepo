use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_extension_activated(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    extension_activated: bool,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE users
        SET extension_activated = $1
        WHERE id = $2
          AND extension_activated IS DISTINCT FROM $1
        RETURNING id
        "#,
        extension_activated,
        user_id
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(result.is_some())
}
