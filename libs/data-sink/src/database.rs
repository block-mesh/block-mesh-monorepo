use secret::Secret;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct UserAndApiToken {
    pub user_id: Uuid,
    pub token: Secret<Uuid>,
    pub email: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "get_user_and_api_token_by_email", skip_all)]
pub async fn get_user_and_api_token_by_email(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
) -> anyhow::Result<Option<UserAndApiToken>> {
    let user: Option<UserAndApiToken> = sqlx::query_as(&format!(
        r#"SELECT
        users.email as email,
        users.id as user_id,
        api_tokens.token,
        users.password
        FROM users
        JOIN api_tokens ON users.id = api_tokens.user_id
        WHERE users.email = '{email}'
        LIMIT 1"#
    ))
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(user)
}
