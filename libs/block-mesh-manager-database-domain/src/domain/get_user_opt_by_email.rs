use crate::domain::option_uuid::OptionUuid;
use crate::domain::user::User;
use crate::domain::user::UserRole;
use secret::Secret;
use sqlx::PgExecutor;

#[tracing::instrument(name = "get_user_opt_by_email", skip_all)]
pub async fn get_user_opt_by_email(
    executor: impl PgExecutor<'_>,
    email: &str,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"SELECT
        id,
        created_at,
        password as "password: Secret<String>",
        email,
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        verified_email
        FROM users WHERE email = $1 LIMIT 1"#,
        email
    )
    .fetch_optional(executor)
    .await
}
