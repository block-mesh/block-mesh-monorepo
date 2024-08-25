use crate::types::metadata::Metadata;
use crate::{BASE_URL, DOWNLOAD_URL};
use anyhow::anyhow;
use reqwest::header::{AsHeaderName, HeaderName};
use reqwest::{header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

static EMPTY_HEADER: HeaderValue = HeaderValue::from_static("");

#[derive(Debug, Serialize, Deserialize)]
pub enum CloudflareMetaHeader {
    Asn,
    City,
    Colo,
    Country,
    Ip,
    Latitude,
    Longitude,
    PostalCode,
    RequestTime,
    Timezone,
}

impl From<&CloudflareMetaHeader> for HeaderName {
    fn from(value: &CloudflareMetaHeader) -> Self {
        HeaderName::from_str(value.to_string().as_str())
            .unwrap_or_else(|_| HeaderName::from_static("CANNOT_CONVERT_HEADER"))
    }
}

impl Display for CloudflareMetaHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cf-meta-{}",
            match self {
                CloudflareMetaHeader::Asn => "asn",
                CloudflareMetaHeader::City => "city",
                CloudflareMetaHeader::Colo => "colo",
                CloudflareMetaHeader::Country => "country",
                CloudflareMetaHeader::Ip => "ip",
                CloudflareMetaHeader::Latitude => "latitude",
                CloudflareMetaHeader::Longitude => "longitude",
                CloudflareMetaHeader::PostalCode => "postalCode",
                CloudflareMetaHeader::RequestTime => "request-time",
                CloudflareMetaHeader::Timezone => "timezone",
            }
        )
    }
}

pub async fn fetch_metadata() -> anyhow::Result<Metadata> {
    let client = Client::new();
    let url = format!("{}/{}{}", BASE_URL, DOWNLOAD_URL, 0);
    let headers = client
        .get(url)
        .send()
        .await
        .map_err(|e| anyhow!("failed to get response - {}", e))?
        .headers()
        .to_owned();

    Ok(Metadata::try_from(headers)?)
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
