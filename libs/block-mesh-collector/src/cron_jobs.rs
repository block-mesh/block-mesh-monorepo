use crate::collector_data::CollectorDailyStats;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde_json::Value;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
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
