use crate::domain::user::User;
use crate::domain::user::UserRole;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get User opt by id", skip(transaction), ret, err)]
pub(crate) async fn get_user_opt_by_id(
    transaction: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        email,
        created_at,
        password as "password: Secret<String>",
        wallet_address,
        role as "role: UserRole"
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
