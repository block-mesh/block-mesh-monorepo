use crate::domain::nonce::Nonce;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get nonce", skip(transaction), ret, err)]
pub async fn get_nonce_by_user_id(
    transaction: &mut Transaction<'_, Postgres>,
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
    .fetch_optional(&mut **transaction)
    .await?)
}
