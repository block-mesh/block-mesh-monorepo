use crate::domain::perk::PerkName;
use chrono::Utc;
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "add_perk_to_user",
    skip(transaction),
    level = "trace",
    ret,
    err
)]
pub(crate) async fn add_perk_to_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: PerkName,
    multiplier: f64,
    one_time_bonus: f64,
    data: Value,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        r#"
        INSERT INTO perks
        (id, user_id, created_at, name, multiplier, one_time_bonus, data)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7)
        "#,
        id,
        user_id,
        now,
        name.to_string(),
        multiplier,
        one_time_bonus,
        data
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
