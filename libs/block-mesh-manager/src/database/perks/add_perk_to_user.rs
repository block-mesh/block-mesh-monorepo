use crate::domain::perk::PerkName;
use chrono::Utc;
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
) -> anyhow::Result<()> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    let _ = sqlx::query!(
        r#"
        INSERT INTO perks
        (id, user_id, created_at, name, multiplier)
        VALUES
        ($1, $2, $3, $4, $5)
        "#,
        id,
        user_id,
        now,
        PerkName::Backpack.to_string(),
        1.1
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
