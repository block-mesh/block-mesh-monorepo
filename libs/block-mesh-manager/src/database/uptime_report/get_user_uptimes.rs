use crate::domain::uptime_report::UptimeReport;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(
    name = "get_user_uptimes",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn get_user_uptimes(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    limit: i64,
) -> anyhow::Result<Vec<UptimeReport>> {
    let uptimes: Vec<UptimeReport> = sqlx::query_as!(
        UptimeReport,
        r#"SELECT
           id,
           nonce,
           user_id,
           created_at,
           ip,
           latitude,
           longitude,
           city,
           region,
           country,
           timezone,
           isp
           FROM uptime_reports
           WHERE user_id = $1
           ORDER BY created_at DESC
           LIMIT $2"#,
        user_id,
        limit
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(uptimes)
}
