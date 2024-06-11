use crate::types::metadata::Metadata;
use crate::{BASE_URL, DOWNLOAD_URL};
use anyhow::anyhow;
use reqwest::{header::HeaderValue, Client};

fn extract_header_value(
    headers: &reqwest::header::HeaderMap,
    header_name: &str,
    na_value: &str,
) -> String {
    headers
        .get(header_name)
        .unwrap_or(&HeaderValue::from_str(na_value).unwrap())
        .to_str()
        .unwrap()
        .to_owned()
}

pub async fn fetch_metadata() -> anyhow::Result<Metadata> {
    let client = Client::new();
    let url = &format!("{}/{}{}", BASE_URL, DOWNLOAD_URL, 0);
    let headers = client
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow!("failed to get response - {}", e))?
        .headers()
        .to_owned();
    Ok(Metadata {
        city: extract_header_value(&headers, "cf-meta-city", "City N/A"),
        country: extract_header_value(&headers, "cf-meta-country", "Country N/A"),
        ip: extract_header_value(&headers, "cf-meta-ip", "IP N/A"),
        asn: extract_header_value(&headers, "cf-meta-asn", "ASN N/A"),
        colo: extract_header_value(&headers, "cf-meta-colo", "Colo N/A"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metadata() {
        let metadata = fetch_metadata().await;
        assert!(metadata.is_ok());
        println!("metadata: {:#?}", metadata.unwrap());
    }
}
