use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct BandwidthReportAvg {
    pub download_speed: Option<f64>,
    pub upload_speed: Option<f64>,
    pub latency: Option<f64>,
}

#[tracing::instrument(
    name = "get_latest_bandwidth_reports",
    skip(transaction),
    ret,
    err,
    level = "trace"
)]
pub(crate) async fn get_latest_bandwidth_reports(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    seconds: i64,
) -> anyhow::Result<BandwidthReportAvg> {
    let now = Utc::now();
    let diff = now - Duration::seconds(seconds);
    let report = sqlx::query_as!(
        BandwidthReportAvg,
        r#"
        SELECT
            AVG(download_speed) as download_speed,
            AVG(upload_speed) as upload_speed,
            AVG(latency) as latency
        FROM bandwidth_reports
        WHERE user_id = $1 AND created_at > $2
        "#,
        user_id,
        diff
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(report)
}
