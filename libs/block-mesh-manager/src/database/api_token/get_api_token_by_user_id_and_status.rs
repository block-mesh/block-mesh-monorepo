use crate::domain::api_token::{ApiToken, ApiTokenStatus};
use secret::Secret;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub async fn get_api_token_by_usr_and_status(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    status: ApiTokenStatus,
) -> anyhow::Result<Option<ApiToken>> {
    Ok(sqlx::query_as!(
        ApiToken,
        r#"SELECT
        id,
        created_at,
        user_id,
        token as "token: Secret<Uuid>",
        status as "status: ApiTokenStatus"
        FROM api_tokens WHERE user_id = $1 and status = $2 LIMIT 1"#,
        user_id,
        status.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?)
}

pub async fn get_api_token_by_usr_and_status_pool(
    pool: &PgPool,
    user_id: &Uuid,
    status: ApiTokenStatus,
) -> anyhow::Result<Option<ApiToken>> {
    Ok(sqlx::query_as!(
        ApiToken,
        r#"SELECT
        id,
        created_at,
        user_id,
        token as "token: Secret<Uuid>",
        status as "status: ApiTokenStatus"
        FROM api_tokens WHERE user_id = $1 and status = $2 LIMIT 1"#,
        user_id,
        status.to_string()
    )
    .fetch_optional(pool)
    .await?)
}
