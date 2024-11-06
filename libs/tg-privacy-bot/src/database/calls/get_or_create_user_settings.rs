use crate::database::models::message_mode::MessageMode;
use crate::database::models::user_settings::UserSettings;
use ai_interface::models::base::ModelName;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_or_create_user_settings(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<UserSettings> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let message_mode = MessageMode::default();
    let model_name = ModelName::default();
    let usage = sqlx::query_as!(
        UserSettings,
        r#"
        INSERT INTO user_settings
        (id, user_id, message_mode, model_name, created_at, updated_at)
        VALUES
        ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id) DO UPDATE SET updated_at = $6
        RETURNING id, user_id, message_mode, model_name, created_at, updated_at
        "#,
        id,
        user_id,
        message_mode.to_string(),
        model_name.to_string(),
        now,
        now.clone()
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(usage)
}
