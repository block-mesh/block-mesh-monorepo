use crate::database::bandwidth::delete_bandwidth_reports_by_time_for_all::delete_bandwidth_reports_by_time_for_all;
use crate::database::uptime_report::delete_uptime_report_by_time_for_all::delete_uptime_report_by_time_for_all;
use crate::database::uptime_report::enrich_uptime_report::enrich_uptime_report;
use crate::errors::error::Error;
use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use block_mesh_common::interfaces::ip_data::{IPData, IpDataPostRequest};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnrichIp {
    pub user_id: Uuid,
    pub uptime_id: Uuid,
    pub ip: String,
}

pub async fn db_cleaner_cron(
    pool: PgPool,
    mut rx: tokio::sync::mpsc::Receiver<EnrichIp>,
) -> Result<(), anyhow::Error> {
    let client = Client::new();
    while let Some(job) = rx.recv().await {
        let mut transaction = pool.begin().await.map_err(Error::from)?;
        let ip_data = client
            .post(BLOCK_MESH_IP_WORKER)
            .json(&IpDataPostRequest { ip: job.ip })
            .send()
            .await?
            .json::<IPData>()
            .await?;
        enrich_uptime_report(&mut transaction, job.uptime_id, ip_data).await?;
        delete_uptime_report_by_time_for_all(&mut transaction, 60 * 60).await?;
        delete_bandwidth_reports_by_time_for_all(&mut transaction, 60 * 60).await?;
        transaction.commit().await.map_err(Error::from)?;
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
    Ok(())
}
