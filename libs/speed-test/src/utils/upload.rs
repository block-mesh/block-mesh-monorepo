use crate::{BASE_URL, UPLOAD_URL};
use anyhow::anyhow;
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
        let response = req_builder
            .send()
            .await
            .map_err(|e| anyhow!("failed to get response - {}", e))?;
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
        let mut counter = 0;
        loop {
            let mbits = test_upload(1_000_000).await;
            if mbits.is_ok() {
                println!("Upload speed: {:.2}mbps", mbits.unwrap());
                break;
            } else {
                counter += 1;
            }
            if counter > 5 {
                panic!("Failed to get upload speed");
            }
        }
    }
}
