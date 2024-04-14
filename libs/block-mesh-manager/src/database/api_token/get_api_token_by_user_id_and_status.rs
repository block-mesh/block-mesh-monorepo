use crate::domain::api_token::{ApiToken, ApiTokenStatus};
use secret::Secret;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Get API token by user and status", skip(transaction), ret, err)]
pub(crate) async fn get_api_token_by_usr_and_status(
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
