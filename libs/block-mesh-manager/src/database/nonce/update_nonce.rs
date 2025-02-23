use block_mesh_manager_database_domain::domain::nonce::Nonce;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_nonce(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let nonce = Nonce::generate_nonce(128);
    sqlx::query!(
        r#"
        UPDATE nonces
        SET nonce = $1
        WHERE user_id = $2"#,
        nonce,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
