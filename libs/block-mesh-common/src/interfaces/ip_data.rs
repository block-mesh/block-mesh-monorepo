pub use ipgeolocate::{Locator, Service};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub name: String,
    pub abuser_score: String,
    pub domain: String,
    pub r#type: String,
    pub network: String,
    pub whois: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataCenter {
    pub datacenter: String,
    pub network: String,
    pub country: String,
    pub region: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asn {
    pub asn: u32,
    pub abuser_score: String,
    pub route: String,
    pub descr: String,
    pub country: String,
    pub active: bool,
    pub org: String,
    pub domain: String,
    pub abuse: String,
    pub r#type: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub rir: String,
    pub whois: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub continent: String,
    pub country: String,
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub zip: String,
    pub timezone: String,
    pub local_time: String,
    pub local_time_unix: u64,
    pub is_dst: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpApiIsResponse {
    pub ip: String,
    pub rir: String,
    pub is_bogon: bool,
    pub is_mobile: bool,
    pub is_crawler: bool,
    pub is_datacenter: bool,
    pub is_tor: bool,
    pub is_proxy: bool,
    pub is_vpn: bool,
    pub is_abuser: bool,
    pub company: Option<Company>,
    pub datacenter: Option<DataCenter>,
    pub asn: Option<Asn>,
    pub location: Option<Location>,
    pub elapsed_ms: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocatorDe {
    /// Returns the IP address.
    pub ip: String,
    /// Latitude of the IP address.
    pub latitude: String,
    /// Longitude of the IP address.
    pub longitude: String,
    /// City of the IP address.
    pub city: String,
    /// Region or state of the IP address.
    pub region: String,
    /// Country of the IP address.
    pub country: String,
    /// Timezone of the IP address.
    pub timezone: String,
    /// ISP of the IP address
    pub isp: String,
}

impl LocatorDe {
    pub fn new(locator: Locator) -> Self {
        Self {
            ip: locator.ip,
            latitude: locator.latitude,
            longitude: locator.longitude,
            city: locator.city,
            region: locator.region,
            country: locator.country,
            timezone: locator.timezone,
            isp: locator.isp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPData {
    pub cf_connecting_ip: Option<String>,
    pub x_real_ip: Option<String>,
    pub x_forwarded_for: Option<String>,
    pub cf_ipcountry: Option<String>,
    pub ip_api_is_response: Option<IpApiIsResponse>,
    pub ip_geolocate_response: Option<LocatorDe>,
}

impl IPData {
    pub fn ip(&self) -> Option<String> {
        if let Some(i) = self.cf_connecting_ip.clone() {
            Some(i)
        } else if let Some(i) = self.ip_geolocate_response.clone() {
            Some(i.ip)
        } else {
            None
        }
    }
}

#[tracing::instrument(name = "get_ip_info", ret, err)]
pub async fn get_ip_info(ip: &str) -> Result<IpApiIsResponse, reqwest::Error> {
    let url = format!("https://api.ipapi.is?q={}", ip);
    let response_result = reqwest::get(&url).await;
    let response = response_result.map_err(|e| {
        tracing::error!("Error getting IP info: {:?}", e);
        e
    })?;
    let response = response.json::<IpApiIsResponse>().await.map_err(|e| {
        tracing::error!("Error deserializing IP info: {:?}", e);
        e
    })?;
    tracing::info!("IP info: {:?}", response);
    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IpDataPostRequest {
    pub ip: String,
}
