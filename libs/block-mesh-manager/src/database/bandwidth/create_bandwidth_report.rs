use block_mesh_common::interfaces::server_api::ReportBandwidthRequest;
use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn create_bandwidth_report(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    report: ReportBandwidthRequest,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO bandwidth_reports
        (id,
         created_at,
         user_id,
         download_speed,
         upload_speed,
         latency,
         country,
         ip,
         asn,
         colo
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#,
        id,
        now,
        user_id,
        report.download_speed,
        report.upload_speed,
        report.latency,
        report.country,
        report.ip,
        report.asn,
        report.colo
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
