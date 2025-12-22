use crate::domain::perk::{Perk, PerkName, PerkTmp};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn add_perk_to_user(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: PerkName,
    multiplier: f64,
    one_time_bonus: f64,
    data: Value,
) -> anyhow::Result<Perk> {
    let now = OffsetDateTime::now_utc();
    let id = Uuid::new_v4();
    let perk: PerkTmp = sqlx::query_as!(
        PerkTmp,
        r#"
        INSERT INTO perks
        (id, user_id, created_at, name, multiplier, one_time_bonus, data, updated_at)
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (user_id, name) DO UPDATE SET updated_at = $8
        RETURNING id, user_id, created_at, name, multiplier, one_time_bonus, data, updated_at
        "#,
        id,
        user_id,
        now,
        name.to_string(),
        multiplier,
        one_time_bonus,
        data,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(Perk {
        id: perk.id.unwrap_or_default(),
        user_id: perk.user_id.unwrap_or_default(),
        created_at: perk.created_at.unwrap_or(OffsetDateTime::UNIX_EPOCH),
        multiplier: perk.multiplier.unwrap_or_default(),
        one_time_bonus: perk.one_time_bonus.unwrap_or_default(),
        name: PerkName::from(perk.name.unwrap_or_default()),
        data: perk.data.unwrap_or_default(),
        updated_at: perk.updated_at.unwrap_or(OffsetDateTime::UNIX_EPOCH),
    })
}
