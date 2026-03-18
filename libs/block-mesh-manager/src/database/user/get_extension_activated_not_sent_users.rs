use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ExtensionActivatedNotSentUser {
    pub user_id: Uuid,
    pub email: String,
    pub wallet_address: Option<String>,
}

#[tracing::instrument(name = "get_extension_activated_not_sent_users", skip_all)]
pub async fn get_extension_activated_not_sent_users(
    pool: &PgPool,
    limit: i64,
) -> anyhow::Result<Vec<ExtensionActivatedNotSentUser>> {
    Ok(sqlx::query_as!(
        ExtensionActivatedNotSentUser,
        r#"
        SELECT
        id AS user_id,
        email,
        wallet_address
        FROM users
        WHERE extension_activated = TRUE
          AND extension_activated_sent = FALSE
        ORDER BY created_at ASC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?)
}
