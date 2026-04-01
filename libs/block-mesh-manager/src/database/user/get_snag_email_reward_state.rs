use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(Debug)]
pub struct SnagEmailRewardState {
    pub pending: bool,
    pub consumed: bool,
}

#[tracing::instrument(name = "get_snag_email_reward_state", skip_all)]
pub async fn get_snag_email_reward_state(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<SnagEmailRewardState> {
    Ok(sqlx::query_as!(
        SnagEmailRewardState,
        r#"
        SELECT
            snag_email_reward_pending AS "pending!",
            snag_email_reward_consumed AS "consumed!"
        FROM users
        WHERE id = $1
        LIMIT 1
        "#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?)
}
