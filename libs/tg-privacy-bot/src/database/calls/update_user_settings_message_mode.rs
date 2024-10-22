use crate::database::models::message_mode::MessageMode;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_user_settings_message_mode(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    message_mode: &MessageMode,
) -> anyhow::Result<()> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE user_settings SET message_mode = $2, updated_at = $3 WHERE id = $1
        "#,
        id,
        message_mode.to_string(),
        now,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
