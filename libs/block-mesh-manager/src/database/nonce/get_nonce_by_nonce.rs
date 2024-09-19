use block_mesh_manager_database_domain::domain::nonce::Nonce;
use secret::Secret;
use sqlx::{Postgres, Transaction};

pub async fn get_nonce_by_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    nonce: &str,
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
        WHERE nonce = $1
        ORDER BY created_at DESC
        LIMIT 1"#,
        nonce
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
