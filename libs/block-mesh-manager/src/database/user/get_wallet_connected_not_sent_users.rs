use crate::utils::snag::snag_cutoff_date;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WalletConnectedNotSentUser {
    pub user_id: Uuid,
    pub email: String,
    pub wallet_address: String,
}

#[tracing::instrument(name = "get_wallet_connected_not_sent_users", skip_all)]
pub async fn get_wallet_connected_not_sent_users(
    pool: &PgPool,
    limit: i64,
) -> anyhow::Result<Vec<WalletConnectedNotSentUser>> {
    let cutoff = snag_cutoff_date();
    Ok(sqlx::query_as!(
        WalletConnectedNotSentUser,
        r#"
        SELECT
        id AS user_id,
        email,
        wallet_address AS "wallet_address!"
        FROM users
        WHERE wallet_address IS NOT NULL
          AND wallet_connected_sent = FALSE
          AND created_at >= $2
        ORDER BY created_at ASC
        LIMIT $1
        "#,
        limit,
        cutoff
    )
    .fetch_all(pool)
    .await?)
}
