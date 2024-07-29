use crate::database::bandwidth::delete_bandwidth_reports_by_time_for_all::delete_bandwidth_reports_by_time_for_all;
use crate::database::uptime_report::delete_uptime_report_by_time_for_all::delete_uptime_report_by_time_for_all;
use crate::database::uptime_report::enrich_uptime_report::enrich_uptime_report;
use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use block_mesh_common::interfaces::ip_data::{IPData, IpDataPostRequest};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::sync::mpsc::UnboundedReceiver;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnrichIp {
    pub user_id: Uuid,
    pub uptime_id: Uuid,
    pub ip: String,
}

pub async fn db_cleaner_cron(
    pool: PgPool,
    mut rx: UnboundedReceiver<EnrichIp>,
) -> Result<(), anyhow::Error> {
    let client = Client::new();
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(16)
        .build()
        .unwrap();

    while let Some(job) = rx.recv().await {
        thread_pool
            .install(|| async {
                let mut transaction = pool.begin().await.unwrap();
                delete_uptime_report_by_time_for_all(&mut transaction, 60 * 60)
                    .await
                    .unwrap();
                delete_bandwidth_reports_by_time_for_all(&mut transaction, 60 * 60)
                    .await
                    .unwrap();
                let ip_data = client
                    .post(BLOCK_MESH_IP_WORKER)
                    .json(&IpDataPostRequest { ip: job.ip })
                    .send()
                    .await
                    .unwrap()
                    .json::<IPData>()
                    .await
                    .unwrap();
                enrich_uptime_report(&mut transaction, job.uptime_id, ip_data)
                    .await
                    .unwrap();
                transaction.commit().await.unwrap();
            })
            .await;
    }
    Ok(())
}
