use crate::{BASE_URL, UPLOAD_URL};
use chrono::Utc;
use reqwest::Client;
use std::cmp;

pub async fn test_upload(payload_size_bytes: usize) -> anyhow::Result<f64> {
    let client = Client::new();
    let url = &format!("{BASE_URL}/{UPLOAD_URL}");
    let payload: Vec<u8> = vec![1; payload_size_bytes];
    let req_builder = client.post(url).body(payload);
    let (_status_code, mbits, _duration) = {
        let start = Utc::now();
        let response = req_builder.send().await.expect("failed to get response");
        let status_code = response.status();
        let end = Utc::now();
        let duration = cmp::max((end - start).num_milliseconds(), 1) as f64;
        let mbits = (payload_size_bytes as f64 * 8.0 / 1_000_000.0) / (duration / 1_000.0);
        (status_code, mbits, duration)
    };
    Ok(mbits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_speed() {
        let mbits = test_upload(1_000_000).await;
        assert!(mbits.is_ok());
        println!("Upload speed: {:.2}mbps", mbits.unwrap());
    }
}
