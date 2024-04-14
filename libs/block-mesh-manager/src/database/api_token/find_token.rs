use crate::domain::api_token::{ApiToken, ApiTokenStatus};
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Find Token", skip(transaction), ret, err)]
pub(crate) async fn find_token(
    transaction: &mut Transaction<'_, Postgres>,
    token: &Uuid,
) -> anyhow::Result<Option<ApiToken>> {
    Ok(sqlx::query_as!(
        ApiToken,
        r#"SELECT
        id,
        created_at,
        user_id,
        token as "token: Secret<Uuid>",
        status as "status: ApiTokenStatus"
        FROM api_tokens WHERE token = $1 and status = $2 LIMIT 1"#,
        token,
        ApiTokenStatus::Active.to_string()
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
