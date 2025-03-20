use crate::domain::perk::{Perk, PerkName, PerkTmp};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn update_user_perk(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    name: PerkName,
    new_data: Value,
    old_data: Value,
) -> anyhow::Result<Perk> {
    let mut merged_data: HashMap<String, Value> = serde_json::from_value(old_data)?;
    let new_map: HashMap<String, Value> = serde_json::from_value(new_data)?;
    for (key, value) in new_map.iter() {
        merged_data.insert(key.to_string(), value.clone());
    }
    let data = serde_json::to_value(merged_data)?;
    let score = block_mesh_common::intract::calc_bonus(data.clone())?;
    let perk: PerkTmp = sqlx::query_as!(
        PerkTmp,
        r#"
        UPDATE perks
        SET
            data = $3,
            one_time_bonus = $4
        WHERE user_id = $1 AND name = $2
        RETURNING id, user_id, created_at, name, multiplier, one_time_bonus, data, updated_at
        "#,
        user_id,
        name.to_string(),
        data,
        score
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(Perk {
        id: perk.id.unwrap_or_default(),
        user_id: perk.user_id.unwrap_or_default(),
        created_at: perk.created_at.unwrap_or_default(),
        multiplier: perk.multiplier.unwrap_or_default(),
        one_time_bonus: perk.one_time_bonus.unwrap_or_default(),
        name: PerkName::from(perk.name.unwrap_or_default()),
        data: perk.data.unwrap_or_default(),
        updated_at: perk.updated_at.unwrap_or_default(),
    })
}
