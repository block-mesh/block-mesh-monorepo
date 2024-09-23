use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_user_password(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    password: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET password= $1 WHERE id = $2"#,
        password,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
