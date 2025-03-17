use crate::domain::perk::PerkName;
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
) -> anyhow::Result<()> {
    let mut merged_data: HashMap<String, Value> = serde_json::from_value(old_data)?;
    let new_map: HashMap<String, Value> = serde_json::from_value(new_data)?;
    for (key, value) in new_map.iter() {
        merged_data.insert(key.to_string(), value.clone());
    }
    let data = serde_json::to_value(merged_data)?;
    let _ = sqlx::query!(
        r#"
        UPDATE perks SET data = $3 WHERE user_id = $1 AND name = $2
        "#,
        user_id,
        name.to_string(),
        data,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
