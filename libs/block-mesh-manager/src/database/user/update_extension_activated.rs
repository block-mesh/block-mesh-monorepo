use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_extension_activated(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    extension_activated: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET extension_activated = $1 WHERE id = $2")
        .bind(extension_activated)
        .bind(user_id)
        .execute(&mut **transaction)
        .await?;
    Ok(())
}
