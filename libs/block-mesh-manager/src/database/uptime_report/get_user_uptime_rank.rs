use sqlx::{Postgres, Transaction};
use uuid::Uuid;

/*
WITH user_totals AS (
    SELECT
        e1.user_id,
        SUM(EXTRACT(EPOCH FROM (e2.created_at - e1.created_at))) AS total_seconds
    FROM
        uptime_reports e1
    JOIN
        uptime_reports e2 ON e1.created_at < e2.created_at
    AND
        ABS(EXTRACT(EPOCH FROM (e1.created_at - e2.created_at))) <= 60
    GROUP BY
        e1.user_id
),
ranked_users AS (
    SELECT
        user_id,
        total_seconds,
        RANK() OVER (ORDER BY total_seconds DESC) AS user_rank
    FROM
        user_totals
)
SELECT
    COALESCE(MAX(user_rank), 0) AS user_rank
FROM
    ranked_users
WHERE
    user_id = $1;
 */

#[tracing::instrument(name = "get_user_uptime_rank", skip(transaction), ret, err)]
pub async fn get_user_uptime_rank(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<i64> {
    let count: Option<i64> = sqlx::query_scalar!(
        r#"
        WITH user_totals AS (
            SELECT
                e1.user_id,
                SUM(EXTRACT(EPOCH FROM (e2.created_at - e1.created_at))) AS total_seconds
            FROM
                uptime_reports e1
            JOIN
                uptime_reports e2 ON e1.created_at < e2.created_at
            AND
                ABS(EXTRACT(EPOCH FROM (e1.created_at - e2.created_at))) <= 60
            GROUP BY
                e1.user_id
        ),
        ranked_users AS (
            SELECT
                user_id,
                total_seconds,
                RANK() OVER (ORDER BY total_seconds DESC) AS user_rank
            FROM
                user_totals
        )
        SELECT
            COALESCE(MAX(user_rank), 0) AS user_rank
        FROM
            ranked_users
        WHERE
            user_id = $1
        "#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default())
}
