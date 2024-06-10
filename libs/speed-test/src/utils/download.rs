use crate::{BASE_URL, DOWNLOAD_URL};
use reqwest::Client;
use std::time::Instant;

pub async fn test_download(payload_size_bytes: usize) -> anyhow::Result<f64> {
    let client = Client::new();
    let url = &format!("{BASE_URL}/{DOWNLOAD_URL}{payload_size_bytes}");
    let req_builder = client.get(url);
    let response = req_builder.send().await.expect("failed to get response");
    let start = Instant::now();
    let _res_bytes = response.bytes();
    let duration = start.elapsed();
    let mbits = (payload_size_bytes as f64 * 8.0 / 1_000_000.0) / duration.as_secs_f64();
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
