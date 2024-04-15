use crate::domain::user::User;
use crate::domain::user::UserRole;
use secret::Secret;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "Get User opt by email", skip(transaction), ret, err)]
pub(crate) async fn get_user_opt_by_email(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        created_at,
        password as "password: Secret<String>",
        email,
        wallet_address,
        role as "role: UserRole"
        FROM users WHERE email = $1 LIMIT 1"#,
        email
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
