use crate::utils::snag::snag_cutoff_date;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PendingSnagEmailRewardUser {
    pub user_id: Uuid,
    pub email: String,
    pub wallet_address: Option<String>,
}

#[tracing::instrument(name = "get_pending_snag_email_reward_users", skip_all)]
pub async fn get_pending_snag_email_reward_users(
    pool: &PgPool,
    limit: i64,
) -> anyhow::Result<Vec<PendingSnagEmailRewardUser>> {
    let cutoff = snag_cutoff_date();

    Ok(sqlx::query_as!(
        PendingSnagEmailRewardUser,
        r#"
        SELECT
            id AS user_id,
            email,
            wallet_address
        FROM users
        WHERE created_at >= $1
          AND snag_email_reward_consumed = FALSE
        ORDER BY created_at ASC
        LIMIT $2
        "#,
        cutoff,
        limit
    )
    .fetch_all(pool)
    .await?)
}
