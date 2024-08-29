use crate::database::bandwidth::delete_bandwidth_reports_by_time_for_all::delete_bandwidth_reports_by_time_for_all;
use crate::database::ip_address::enrich_ip_address::enrich_ip_address;
use crate::database::ip_address::get_or_create_ip_address::get_or_create_ip_address;
use crate::database::uptime_report::delete_uptime_report_by_time_for_all::delete_uptime_report_by_time_for_all;
use crate::database::users_ip::get_or_create_users_ip::get_or_create_users_ip;
use crate::errors::error::Error;
use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use block_mesh_common::interfaces::ip_data::{IPData, IpDataPostRequest};
use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichIp {
    pub user_id: Uuid,
    pub ip: String,
}

pub async fn db_cleaner_cron(
    pool: PgPool,
    mut rx: UnboundedReceiver<EnrichIp>,
) -> Result<(), anyhow::Error> {
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default();
    let thread_pool = rayon::ThreadPoolBuilder::new().num_threads(16).build()?;

    while let Some(job) = rx.recv().await {
        let pool = pool.clone();
        let client = client.clone();
        thread_pool
            .install(|| async {
                let _ = enrich_ip_and_cleanup(pool, client, job).await;
            })
            .await;
    }
    Ok(())
}

pub async fn enrich_ip_and_cleanup(
    pool: PgPool,
    client: Client,
    job: EnrichIp,
) -> anyhow::Result<()> {
    let pool = pool.clone();
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    delete_uptime_report_by_time_for_all(&mut transaction, 3600)
        .await
        .map_err(Error::from)?;
    delete_bandwidth_reports_by_time_for_all(&mut transaction, 3600)
        .await
        .map_err(Error::from)?;
    let ip_address = get_or_create_ip_address(&mut transaction, &job.ip)
        .await
        .map_err(Error::from)?;
    get_or_create_users_ip(&mut transaction, &job.user_id, &ip_address.id)
        .await
        .map_err(Error::from)?;
    if !ip_address.enriched {
        let ip_data = client
            .post(BLOCK_MESH_IP_WORKER)
            .json(&IpDataPostRequest { ip: job.ip })
            .send()
            .await?
            .json::<IPData>()
            .await?;
        enrich_ip_address(&mut transaction, ip_address.id, &ip_data).await?;
    }
    transaction.commit().await?;
    Ok(())
}
