use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "apply_ref_bonus_for_dail_stat", skip_all)]
pub async fn apply_ref_bonus_for_dail_stat(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    bonus: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE daily_stats
        SET
            ref_bonus = $1,
            ref_bonus_applied = true
         WHERE
            id = $2
            AND ref_bonus_applied = false
         "#,
        bonus,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
