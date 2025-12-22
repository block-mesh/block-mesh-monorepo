use crate::collector_data::{CollectorDailyStats, CollectorData, ExportData};
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde_json::Value;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::time::sleep;

#[tracing::instrument(name = "get_items", skip_all, err)]
pub async fn get_items() -> anyhow::Result<Vec<String>> {
    let asins_file = env::var("ASINS_FILE")?;
    let client = reqwest::Client::new();
    let response = client
        .get(asins_file)
        .header("User-Agent", "block-mesh-collector")
        .send()
        .await?;
    if response.status().is_success() {
        let items: Vec<String> = response.json().await?;
        // tracing::info!("items => {:?}", items);
        Ok(items)
    } else {
        Err(anyhow::anyhow!("Failed to fetch items"))
    }
}

#[tracing::instrument(name = "get_product", skip_all, err)]
pub async fn get_product(asin: &str) -> anyhow::Result<()> {
    let worker_url = env::var("WORKER_URL")?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/{}", worker_url, asin))
        .header("User-Agent", "block-mesh-collector")
        .send()
        .await?;
    if response.status().is_success() {
        let _: Value = response.json().await?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to fetch product data"))
    }
}

pub async fn upload_to_r2(pool: Arc<PgPool>, client: Arc<Client>) -> anyhow::Result<()> {
    let sleep_duration = Duration::from_secs(60 * 60 * 12);
    let limit = env::var("R2_LIMIT")
        .unwrap_or("10000".to_string())
        .parse::<i64>()
        .unwrap_or(10_000);
    loop {
        let now = OffsetDateTime::now_utc();
        let day = now.date();
        if let Ok(mut transaction) = create_txn(&pool).await {
            if let Ok(items) = CollectorData::get_day_data(&mut transaction, day, limit).await {
                let export_items: Vec<ExportData> = items
                    .iter()
                    .filter_map(|i| match i.extract_for_export() {
                        Ok(e) => Some(e),
                        Err(error) => {
                            tracing::error!("Cant export {}", error);
                            None
                        }
                    })
                    .collect();
                tracing::info!("export_items = {:#?}", export_items);
                if let Ok(data_bytes) = serde_json::to_string(&export_items) {
                    let byte_stream = ByteStream::from(data_bytes.into_bytes()); // String -> Vec<u8> -> ByteStream
                    let key = format!("{}.json", day);
                    let _ = client
                        .put_object()
                        .bucket("amazon")
                        .key(key)
                        .body(byte_stream)
                        .send()
                        .await;
                }
            }
            let _ = commit_txn(transaction).await;
        }
        sleep(sleep_duration).await;
    }
}

pub async fn get_products(pool: Arc<PgPool>) -> anyhow::Result<()> {
    let sleep_duration = Duration::from_millis(
        env::var("SLEEP_DURATION")
            .unwrap_or("10000".to_string())
            .parse::<u64>()
            .unwrap_or(10000),
    );
    let daily_limit = env::var("DAILY_LIMIT")
        .unwrap_or("10000".to_string())
        .parse::<i32>()
        .unwrap_or(10_000);
    let mut items = get_items().await?;
    loop {
        if let Ok(mut transaction) = create_txn(&pool).await {
            if let Ok(daily_stats_collector) =
                CollectorDailyStats::get_or_create_collector_daily_stats(&mut transaction).await
            {
                if daily_stats_collector.count < daily_limit {
                    if let Some(asin) = items.pop() {
                        if let Err(e) = get_product(&asin).await {
                            eprintln!("Error fetching product {}: {}", asin, e);
                        }
                    }
                }
            }
            let _ = commit_txn(transaction).await;
        }
        sleep(sleep_duration).await;
    }
}
