use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_snag_email_reward_state", skip_all)]
pub async fn update_snag_email_reward_state(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    pending: bool,
    consumed: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET snag_email_reward_pending = $1,
            snag_email_reward_consumed = $2
        WHERE id = $3
          AND (
              snag_email_reward_pending IS DISTINCT FROM $1
              OR snag_email_reward_consumed IS DISTINCT FROM $2
          )
        "#,
        pending,
        consumed,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
