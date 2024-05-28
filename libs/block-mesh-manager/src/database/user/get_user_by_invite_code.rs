use crate::domain::option_uuid::OptionUuid;
use crate::domain::user::User;
use crate::domain::user::UserRole;
use secret::Secret;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "get_user_opt_by_invited_code", skip(transaction), ret, err)]
pub(crate) async fn get_user_opt_by_invited_code(
    transaction: &mut Transaction<'_, Postgres>,
    invite_code: String,
) -> anyhow::Result<Option<User>> {
    Ok(sqlx::query_as!(
        User,
        r#"SELECT
        id,
        email,
        created_at,
        password as "password: Secret<String>",
        wallet_address,
        role as "role: UserRole",
        invited_by as "invited_by: OptionUuid",
        invite_code
        FROM users WHERE invite_code = $1 LIMIT 1"#,
        invite_code
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
