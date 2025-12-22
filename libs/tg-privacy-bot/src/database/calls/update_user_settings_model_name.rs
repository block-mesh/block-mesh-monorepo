use ai_interfaces::models::base::ModelName;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn update_user_settings_model_name(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    model_name: &ModelName,
) -> anyhow::Result<()> {
    let now = OffsetDateTime::now_utc();
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
