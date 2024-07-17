use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "update_aggregate",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn update_aggregate(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
    value: &serde_json::Value,
) -> anyhow::Result<Uuid> {
    sqlx::query!(
        r#"UPDATE aggregates SET value = $1 WHERE id = $2"#,
        value,
        id,
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
