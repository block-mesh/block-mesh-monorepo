use block_mesh_common::interfaces::server_api::LeaderBoardUser;
use chrono::{Duration, NaiveDate, Utc};
use dashmap::DashMap;
use sqlx::{Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::OnceCell;

pub async fn get_daily_leaderboard(
    transaction: &mut Transaction<'_, Postgres>,
    uptime_factor: f64,
    tasks_factor: f64,
    limit: i64,
) -> anyhow::Result<Vec<LeaderBoardUser>> {
    let day = Utc::now().date_naive() - Duration::days(1);
    let daily_stats = sqlx::query_as!(
        LeaderBoardUser,
        r#"
        SELECT
            users.email AS email,
            (uptime * $1 + CAST(tasks_count as DOUBLE PRECISION) * $2) AS points
        FROM
	        daily_stats
	        JOIN users ON users.id = daily_stats.user_id
        WHERE day = $3
        ORDER BY points DESC
        LIMIT $4
        "#,
        // r#"
        // SELECT
        // 	users.email AS email,
        // 	(uptime * $1 + CAST(tasks_count as DOUBLE PRECISION) * $2) AS points,
        // 	COUNT(users_ip.id) AS ips
        // FROM
        // 	daily_stats
        // 	JOIN users ON users.id = daily_stats.user_id
        // 	JOIN users_ip ON users.id = users_ip.user_id
        // WHERE
        //     day = $3
        //     AND users_ip.updated_at >= NOW() - INTERVAL '24 hours'
        // GROUP BY
        //     users.email,
        //     points
        // HAVING
        //     COUNT(users_ip.id) < 10
        // ORDER BY
        // 	points DESC
        // 	LIMIT $4
        // "#,
        uptime_factor,
        tasks_factor,
        day,
        limit
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(daily_stats)
}
