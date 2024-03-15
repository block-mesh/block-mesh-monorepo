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
