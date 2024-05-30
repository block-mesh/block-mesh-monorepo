use ipapi_is_rust::get_ip_info;
use ipapi_is_rust::response::IpApiIsResponse;
use ipgeolocate::{Locator, Service};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

static IP_HEADERS: [&str; 4] = [
    "cf-connecting-ip",
    "x-real-ip",
    "x-forwarded-for",
    "cf-ipcountry",
];

#[derive(Debug, Serialize, Deserialize)]
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
struct IPData {
    cf_connecting_ip: Option<String>,
    x_real_ip: Option<String>,
    x_forwarded_for: Option<String>,
    cf_ipcountry: Option<String>,
    ip_api_is_response: Option<IpApiIsResponse>,
    ip_geolocate_response: Option<LocatorDe>,
}

static IP_GEO_LOCATE_SERVICES: [Service; 4] = [
    Service::IpWhois,
    Service::IpApi,
    Service::IpApiCo,
    Service::FreeGeoIp,
];

impl IPData {
    #[tracing::instrument(name = "IPData::new")]
    pub fn new(headers: FxHashMap<String, String>) -> Self {
        Self {
            cf_connecting_ip: headers.get("cf-connecting-ip").map(|s| s.to_string()),
            x_real_ip: headers.get("x-real-ip").map(|s| s.to_string()),
            x_forwarded_for: headers.get("x-forwarded-for").map(|s| s.to_string()),
            cf_ipcountry: headers.get("cf-ipcountry").map(|s| s.to_string()),
            ip_api_is_response: None,
            ip_geolocate_response: None,
        }
    }

    #[tracing::instrument(name = "IPData::get_ip_api_is_response", skip(self))]
    pub async fn get_ip_api_is_response(&mut self) {
        let ip = [
            self.cf_connecting_ip.as_ref(),
            self.x_real_ip.as_ref(),
            self.x_forwarded_for.as_ref(),
        ];
        let ip = ip.iter().find(|ip| ip.is_some());
        if let Some(ip) = ip {
            let ip = ip.as_ref().unwrap();
            let response = get_ip_info(ip).await;
            match response {
                Ok(response) => {
                    self.ip_api_is_response = Some(response);
                }
                Err(e) => {
                    tracing::error!("Error getting IP info: {:?}", e);
                }
            }
            for service in IP_GEO_LOCATE_SERVICES {
                let response = Locator::get(ip, service).await;
                match response {
                    Ok(response) => {
                        self.ip_geolocate_response = Some(LocatorDe::new(response));
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Error getting IP info: {:?}", e);
                    }
                }
            }
        }
    }
}

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[derive(Debug, Serialize, Deserialize)]
struct PostRequest {
    ip: String,
}

#[event(fetch)]
async fn main(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let mut headers: FxHashMap<String, String> = FxHashMap::default();
    if req.method() == Method::Get {
        req.headers().entries().for_each(|(k, v)| {
            if IP_HEADERS.contains(&k.as_str()) {
                headers.insert(k.clone(), v.clone());
            }
        });
    } else if req.method() == Method::Post {
        let body = req.json::<PostRequest>().await?;
        headers.insert(IP_HEADERS[0].to_string(), body.ip);
    } else {
        return Response::error("Method not allowed", 405);
    }
    let mut ip_data = IPData::new(headers);
    ip_data.get_ip_api_is_response().await;
    tracing::info!("IP Data: {:?}", ip_data);
    Response::from_json(&ip_data)
}
