use crate::domain::perk::Perk;
use sqlx::{query_as, Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
struct Id {
    id: Uuid,
}

#[tracing::instrument(name = "get_user_perks", skip(transaction), ret, err, level = "trace")]
pub async fn get_user_perks(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Vec<Perk>> {
    let perks = query_as!(
        Perk,
        r#"
        SELECT
        id, user_id, name, created_at, multiplier, one_time_bonus, data
        FROM perks
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(perks)
}
