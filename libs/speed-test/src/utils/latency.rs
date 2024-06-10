use crate::{BASE_URL, DOWNLOAD_URL};
use chrono::Utc;
use regex::Regex;
use reqwest::Client;
use std::cmp;

pub async fn test_latency() -> anyhow::Result<f64> {
    let client = Client::new();
    let url = &format!("{}/{}{}", BASE_URL, DOWNLOAD_URL, 0);
    let req_builder = client.get(url);
    let start = Utc::now();
    let response = req_builder.send().await.expect("failed to get response");
    let _status_code = response.status();
    let end = Utc::now();
    let duration = cmp::max((end - start).num_milliseconds(), 1) as f64;

    let re = Regex::new(r"cfRequestDuration;dur=([\d.]+)").unwrap();
    let cf_req_duration: f64 = re
        .captures(
            response
                .headers()
                .get("Server-Timing")
                .expect("No Server-Timing in response header")
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    let mut req_latency = duration - cf_req_duration;
    if req_latency < 0.0 {
        // TODO investigate negative latency values
        req_latency = 0.0
    }
    Ok(req_latency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_latency_speed() {
        let latency = test_latency().await;
        assert!(latency.is_ok());
        println!("Latency: {:.2}ms", latency.unwrap());
    }
}
