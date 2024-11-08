use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn update_api_token(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let token = Uuid::new_v4();
    sqlx::query!(
        r#"UPDATE api_tokens SET token = $1 WHERE user_id = $2"#,
        token,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
