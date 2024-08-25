use crate::metadata::CloudflareMetaHeader;
use anyhow::Context;
use reqwest::header::{HeaderMap, HeaderName};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Metadata {
    pub city: String,
    pub country: String,
    pub ip: String,
    pub asn: String,
    pub colo: String,
}

impl TryFrom<&HeaderMap> for Metadata {
    type Error = anyhow::Error;
    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        use crate::utils::metadata::CloudflareMetaHeader as H;
        let city = value.get_cloudflare(H::City)?;
        let country = value.get_cloudflare(H::Country)?;
        let ip = value.get_cloudflare(H::Ip)?;
        let asn = value.get_cloudflare(H::Asn)?;
        let colo = value.get_cloudflare(H::Colo)?;

        Ok(Self {
            city,
            country,
            ip,
            asn,
            colo,
        })
    }
}

impl TryFrom<HeaderMap> for Metadata {
    type Error = anyhow::Error;
    fn try_from(value: HeaderMap) -> Result<Self, Self::Error> {
        Metadata::try_from(&value)
    }
}

trait CloudflareHeaderMap {
    fn get_cloudflare(&self, header: CloudflareMetaHeader) -> anyhow::Result<String>;
}

impl CloudflareHeaderMap for HeaderMap {
    fn get_cloudflare(&self, header: CloudflareMetaHeader) -> anyhow::Result<String> {
        Ok(self
            .get(HeaderName::from(&header))
            .context(format!("Header {} not found", header))?
            .to_str()?
            .to_owned())
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            city: "".to_string(),
            country: "".to_string(),
            ip: "".to_string(),
            asn: "".to_string(),
            colo: "".to_string(),
        }
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "City: {}\nCountry: {}\nIp: {}\nAsn: {}\nColo: {}",
            self.city, self.country, self.ip, self.asn, self.colo
        )
    }
}
