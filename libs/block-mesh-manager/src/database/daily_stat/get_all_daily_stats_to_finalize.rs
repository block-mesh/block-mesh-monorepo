use sqlx::{query_as, Postgres, Transaction};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

#[allow(dead_code)]
struct Id {
    id: Uuid,
}

pub async fn get_all_daily_stats_to_finalize(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Uuid>> {
    let now = OffsetDateTime::now_utc() - Duration::days(1);
    let day = now.date();
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
