use chrono::{Duration, Utc};
use sqlx::{query_as, Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
struct Id {
    id: Uuid,
}

#[tracing::instrument(
    name = "get_all_daily_stats_to_finalize",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub async fn get_all_daily_stats_to_finalize(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Uuid>> {
    let now = Utc::now() - Duration::days(1);
    let day = now.date_naive();
    let rows: Vec<_> = query_as!(
        Id,
        r#"
        SELECT
        id
        FROM daily_stats
        WHERE day < $1
        LIMIT 10000
        "#,
        day
    )
    .fetch_all(&mut **transaction)
    .await?;
    let ids = rows.iter().map(|i| i.id).collect();
    Ok(ids)
}
