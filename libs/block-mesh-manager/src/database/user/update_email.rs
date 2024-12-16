use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_email(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET email= $1 WHERE id = $2"#,
        email,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
