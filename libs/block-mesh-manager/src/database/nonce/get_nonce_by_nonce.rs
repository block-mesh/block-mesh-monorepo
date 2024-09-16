use crate::domain::nonce::Nonce;
use secret::Secret;
use sqlx::PgPool;

#[tracing::instrument(name = "Get nonce by nonce by pool", skip_all, ret, err)]
pub async fn get_nonce_by_nonce_pool(pool: &PgPool, nonce: &str) -> anyhow::Result<Option<Nonce>> {
    Ok(sqlx::query_as!(
        Nonce,
        r#"
        SELECT
        id,
        created_at,
        user_id,
        nonce as "nonce: Secret<String>"
        FROM nonces
        WHERE nonce = $1
        ORDER BY created_at DESC
        LIMIT 1"#,
        nonce
    )
    .fetch_optional(pool)
    .await?)
}
