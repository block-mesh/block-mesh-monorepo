use block_mesh_manager_database_domain::domain::api_token::{ApiToken, ApiTokenStatus};
use secret::Secret;
use sqlx::PgExecutor;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn get_api_token_by_usr_and_status(
    executor: impl PgExecutor<'_>,
    user_id: &Uuid,
    status: ApiTokenStatus,
) -> sqlx::Result<Option<ApiToken>> {
    sqlx::query_as!(
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
    .fetch_optional(executor)
    .await
}
