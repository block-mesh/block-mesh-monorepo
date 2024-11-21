pub use ipgeolocate::{Locator, Service};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub name: Option<String>,
    pub abuser_score: Option<String>,
    pub domain: Option<String>,
    pub r#type: Option<String>,
    pub network: Option<String>,
    pub whois: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataCenter {
    pub datacenter: Option<String>,
    pub network: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asn {
    pub asn: u32,
    pub abuser_score: Option<String>,
    pub route: Option<String>,
    pub descr: Option<String>,
    pub country: Option<String>,
    pub active: Option<bool>,
    pub org: Option<String>,
    pub domain: Option<String>,
    pub abuse: Option<String>,
    pub r#type: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub rir: Option<String>,
    pub whois: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub continent: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub zip: Option<String>,
    pub timezone: Option<String>,
    pub local_time: Option<String>,
    pub local_time_unix: Option<u64>,
    pub is_dst: Option<bool>,
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

    pub fn asn(&self) -> Option<u64> {
        if let Some(ip_api) = &self.ip_api_is_response {
            if let Some(asn) = &ip_api.asn {
                return Some(asn.asn as u64);
            }
        }
        None
    }

    pub fn is_datacenter(&self) -> Option<bool> {
        if let Some(ip_api) = &self.ip_api_is_response {
            return Some(ip_api.is_datacenter);
        }
        None
    }

    pub fn is_vps(&self, asn_list: Vec<u64>) -> Option<bool> {
        if let Some(asn) = self.asn() {
            if asn_list.contains(&asn) {
                return Some(true);
            }
        }
        if let Some(is_datacenter) = self.is_datacenter() {
            if is_datacenter {
                return Some(true);
            }
        }
        None
    }
}

#[tracing::instrument(name = "get_ip_info", ret, err)]
pub async fn get_ip_info(ip: &str) -> anyhow::Result<IpApiIsResponse> {
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
