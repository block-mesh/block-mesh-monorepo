use chrono::{Duration, Utc};
use num_traits::cast::ToPrimitive;
use sqlx::types::BigDecimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_user_uptime_in_duration(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    duration: u64,
) -> anyhow::Result<f64> {
    let since = Utc::now() - Duration::seconds(duration as i64);
    let count: Option<BigDecimal> = sqlx::query_scalar!(
        r#"
            SELECT
                SUM(EXTRACT(EPOCH FROM (e2.created_at - e1.created_at))) AS total_seconds_get_user_uptime_in_duration
            FROM
                uptime_reports e1
            JOIN
                uptime_reports e2 ON e1.created_at < e2.created_at
            AND
                ABS(EXTRACT(EPOCH FROM (e1.created_at - e2.created_at))) <= 60
            AND
                e1.user_id = $1
            AND
                e2.user_id = $1
            AND
                e1.created_at >= $2
            AND
                e2.created_at >= $2
        "#,
        user_id,
        since
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(count.unwrap_or_default().to_f64().unwrap_or_default())
}
