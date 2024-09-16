use crate::domain::nonce::Nonce;
use secret::Secret;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Get nonce via pool", skip_all, ret, err, level = "trace")]
pub async fn get_nonce_by_user_id_pool(
    pool: &PgPool,
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
    .fetch_optional(pool)
    .await?)
}
