use chrono::Utc;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Nonce", skip(transaction), ret, err)]
pub async fn create_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    nonce: &Secret<String>,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO nonces
        (id, created_at, user_id, nonce)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        now,
        user_id,
        nonce.as_ref(),
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
