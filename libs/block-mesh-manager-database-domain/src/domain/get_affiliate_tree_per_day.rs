use block_mesh_common::interfaces::server_api::{TmpUserAffiliate, UserAffiliate};
use chrono::NaiveDate;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_affiliate_tree_per_day(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    day: &NaiveDate,
) -> anyhow::Result<Vec<UserAffiliate>> {
    let affiliates: Vec<TmpUserAffiliate> = sqlx::query_as!(
        TmpUserAffiliate,
        r#"
        WITH RECURSIVE invite_tree AS (
    -- Base case: Start with the specified root user at level 0
    SELECT
        id AS user_id,
        email,
        invited_by,
        verified_email,
        proof_of_humanity,
        0 AS level
    FROM users
    WHERE id = $1 -- Replace with the specific user ID
    UNION ALL
    -- Recursive case: Find invites of invites, limited to 3 levels (0, 1, 2)
    SELECT
        u.id AS user_id,
        u.email,
        u.invited_by,
        u.verified_email,
        u.proof_of_humanity,
        it.level + 1 AS level
    FROM users u
    JOIN invite_tree it ON u.invited_by = it.user_id
    WHERE it.level < 3
      AND u.verified_email = true
      AND u.proof_of_humanity = true
)
SELECT
    i.user_id,
    i.email,
    i.invited_by,
    i.verified_email,
    i.proof_of_humanity,
    i.level,
    ds.uptime,
    ds.tasks_count
FROM invite_tree i
LEFT JOIN daily_stats ds ON ds.user_id = i.user_id
AND ds.day = $2
        "#,
        user_id,
        day
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(affiliates
        .into_iter()
        .map(|i| UserAffiliate {
            user_id: i.user_id.unwrap_or_default(),
            email: i.email.unwrap_or_default(),
            invited_by: i.invited_by.unwrap_or_default(),
            verified_email: i.verified_email.unwrap_or_default(),
            proof_of_humanity: i.proof_of_humanity.unwrap_or_default(),
            level: i.level.unwrap_or_default(),
            uptime: i.uptime.unwrap_or_default(),
            tasks_count: i.tasks_count.unwrap_or_default(),
        })
        .collect())
}
