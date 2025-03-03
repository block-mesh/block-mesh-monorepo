use crate::domain::user::UserAndApiToken;
use chrono::{Duration, Utc};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use secret::Secret;
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;

type CacheType = Arc<RwLock<HashMapWithExpiry<String, Option<UserAndApiToken>>>>;
static CACHE: OnceCell<CacheType> = OnceCell::const_new();

#[tracing::instrument(name = "get_user_and_api_token_by_email", skip_all)]
pub async fn get_user_and_api_token_by_email(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
) -> anyhow::Result<Option<UserAndApiToken>> {
    let cache = CACHE
        .get_or_init(|| async { Arc::new(RwLock::new(HashMapWithExpiry::new())) })
        .await;
    if let Some(out) = cache.read().await.get(&email.to_string()).await {
        return Ok(out);
    }
    let output = sqlx::query_as!(
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
    .await?;
    let date = Utc::now() + Duration::milliseconds(600_000);
    cache
        .write()
        .await
        .insert(email.to_string(), output.clone(), Some(date))
        .await;
    Ok(output)
}
