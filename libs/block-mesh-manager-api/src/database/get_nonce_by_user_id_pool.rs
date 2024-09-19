use block_mesh_manager_database_domain::domain::nonce::Nonce;
use secret::Secret;
use sqlx::{PgExecutor, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn get_nonce_by_user_id_pool(
    executor: impl PgExecutor<'_>,
    user_id: &Uuid,
) -> anyhow::Result<Option<Nonce>> {
    Ok(sqlx::query_as!(
        Nonce,
        r#"
        SELECT
        id,
        created_at,
        user_id,
        nonce as "nonce: Secret<String>"
        FROM nonces
        WHERE user_id = $1
        ORDER BY created_at DESC
        LIMIT 1"#,
        user_id
    )
    .fetch_optional(executor)
    .await?)
}
