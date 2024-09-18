use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Debug)]
pub struct UserUptime {
    pub user_id: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<f64>,
}

pub async fn get_user_latest_uptime(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> anyhow::Result<Option<UserUptime>> {
    Ok(sqlx::query_as!(
        UserUptime,
        r#"
        WITH time_diffs AS (
            SELECT
                id,
                created_at,
                user_id,
                lag(created_at) OVER (ORDER BY created_at) AS prev_created_at
            FROM
                uptime_reports
            WHERE
                user_id = $1
        ),
        grouped_times AS (
            SELECT
                id,
                created_at,
                user_id,
                prev_created_at,
                CASE
                    WHEN EXTRACT(EPOCH FROM (created_at - prev_created_at)) <= 60 THEN 0
                    ELSE 1
                END AS time_gap
            FROM
                time_diffs
        ),
        group_ids AS (
            SELECT
                id,
                created_at,
                user_id,
                prev_created_at,
                SUM(time_gap) OVER (ORDER BY created_at) AS group_id
            FROM
                grouped_times
        ),
        latest_group AS (
            SELECT
                MAX(group_id) AS latest_group_id
            FROM
                group_ids
        )
    SELECT
        gt.user_id,
        MIN(gt.created_at) AS start_time,
        MAX(gt.created_at) AS end_time,
        CAST(EXTRACT(EPOCH FROM MAX(gt.created_at) - MIN(gt.created_at)) AS DOUBLE PRECISION) AS duration_seconds
    FROM
        group_ids gt
    JOIN
        latest_group lg
    ON
        gt.group_id = lg.latest_group_id
    WHERE
        gt.user_id = $1
    GROUP BY
        gt.user_id, gt.group_id
    ORDER BY
        gt.user_id
        "#,
        user_id
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
