use crate::{BASE_URL, DOWNLOAD_URL};
use anyhow::anyhow;
use chrono::Utc;
use reqwest::Client;
use std::cmp;

pub async fn test_download(payload_size_bytes: usize) -> anyhow::Result<f64> {
    let client = Client::new();
    let url = &format!("{BASE_URL}/{DOWNLOAD_URL}{payload_size_bytes}");
    let req_builder = client.get(url);
    let response = req_builder
        .send()
        .await
        .map_err(|e| anyhow!("failed to get response - {}", e))?;
    let start = Utc::now();
    let _res_bytes = response.bytes().await;
    let end = Utc::now();
    let duration = cmp::max((end - start).num_milliseconds(), 1) as f64;
    let mbits = (payload_size_bytes as f64 * 8.0 / 1_000_000.0) / (duration / 1_000.0);
    Ok(mbits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_speed() {
        let mbits = test_download(1_000_000).await;
        assert!(mbits.is_ok());
        println!("Download speed: {:.2}mbps", mbits.unwrap());
    }
}
