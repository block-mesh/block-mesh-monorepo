use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "update_aggregate", skip(pool), ret, err, level = "trace")]
pub(crate) async fn update_aggregate(
    pool: &PgPool,
    id: Uuid,
    value: &serde_json::Value,
) -> anyhow::Result<Uuid> {
    sqlx::query!(
        r#"UPDATE aggregates SET value = $1 WHERE id = $2"#,
        value,
        id,
    )
    .execute(pool)
    .await?;
    Ok(id)
}
