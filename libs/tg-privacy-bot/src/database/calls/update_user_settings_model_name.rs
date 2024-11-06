use ai_interface::models::base::ModelName;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_user_settings_model_name(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    model_name: &ModelName,
) -> anyhow::Result<()> {
    let now = Utc::now();
    sqlx::query!(
        r#"
        UPDATE user_settings SET model_name = $2, updated_at = $3 WHERE id = $1
        "#,
        id,
        model_name.to_string(),
        now,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
