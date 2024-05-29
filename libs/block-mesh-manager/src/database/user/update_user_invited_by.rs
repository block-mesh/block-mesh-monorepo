use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "update_user_invited_by", skip(transaction), ret, err)]
pub(crate) async fn update_user_invited_by(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    invited_by_user_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET invited_by = $1 WHERE id = $2"#,
        invited_by_user_id,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
