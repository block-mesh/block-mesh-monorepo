use block_mesh_common::constants::DeviceType;
use block_mesh_common::reqwest::http_client;
use chrono::Utc;
use clickhouse::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Params {
    mentions: u64,
    code: String,
}

#[tracing::instrument(name = "update_network_stats", level = "trace", skip_all)]
pub async fn update_network_stats(clickhouse_client: Arc<Client>) -> Result<(), anyhow::Error> {
    let stats_url = env::var("STATS_URL")?;
    let code = env::var("ADMIN_PARAM")?;
    let app_env = env::var("APP_ENVIRONMENT")?;
    loop {
        let code = code.clone();
        let now = Utc::now();
        let day = now - Duration::from_secs(60 * 60 * 24 * 2);
        let day = day.date_naive();
        tracing::info!("data = {day}");
        let clickhouse_client = clickhouse_client.clone();
        let q = if app_env == "local" {
            format!(
                r#"SELECT count(*) FROM data_sinks_clickhouse WHERE created_at::DATE = '{day}'"#
            )
        } else {
            format!(
                r#"
                SELECT count(*) FROM data_sinks_clickhouse WHERE event_date = '{day}'
            "#
            )
        };
        if let Ok(output) = clickhouse_client.query(&q).fetch_one::<u64>().await {
            let client = http_client(DeviceType::AppServer);
            let _ = client
                .get(&stats_url)
                .query(&Params {
                    code: code,
                    mentions: output,
                })
                .send()
                .await;
        }
        tokio::time::sleep(Duration::from_secs(60 * 60 * 8)).await;
    }
}
