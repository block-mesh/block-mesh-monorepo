use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub(crate) async fn update_user_wallet(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    wallet: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE users SET wallet_address = $1 WHERE id = $2"#,
        wallet,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
