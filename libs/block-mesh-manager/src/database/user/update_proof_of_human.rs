use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_proof_of_human(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    proof_of_human: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET proof_of_humanity = $1 WHERE id = $2"#,
        proof_of_human,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
