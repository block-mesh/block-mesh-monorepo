use crate::domain::user::UserAndApiToken;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "get_user_and_api_token_by_email", skip_all)]
pub async fn get_user_and_api_token_by_email(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
) -> anyhow::Result<Option<UserAndApiToken>> {
    Ok(sqlx::query_as!(
        UserAndApiToken,
        r#"SELECT
        users.email as email,
        users.id as user_id,
        api_tokens.token as "token: Secret<Uuid>",
        users.password as "password: Secret<String>",
        users.wallet_address as wallet_address,
        users.verified_email as verified_email
        FROM users
        JOIN api_tokens ON users.id = api_tokens.user_id
        WHERE users.email = $1
        LIMIT 1"#,
        email,
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
