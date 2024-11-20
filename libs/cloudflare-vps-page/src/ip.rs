use block_mesh_common::interfaces::ip_data::{get_ip_info, IPData, Locator, LocatorDe, Service};
use rustc_hash::FxHashMap;

static IP_GEO_LOCATE_SERVICES: [Service; 4] = [
    Service::IpWhois,
    Service::IpApi,
    Service::IpApiCo,
    Service::FreeGeoIp,
];

#[tracing::instrument(name = "IPData::new")]
pub fn new_ipdata(headers: FxHashMap<String, String>) -> IPData {
    IPData {
        cf_connecting_ip: headers.get("cf-connecting-ip").map(|s| s.to_string()),
        x_real_ip: headers.get("x-real-ip").map(|s| s.to_string()),
        x_forwarded_for: headers.get("x-forwarded-for").map(|s| s.to_string()),
        cf_ipcountry: headers.get("cf-ipcountry").map(|s| s.to_string()),
        ip_api_is_response: None,
        ip_geolocate_response: None,
    }
}

#[tracing::instrument(name = "IPData::get_ip_api_is_response")]
pub async fn get_ip_api_is_response(ip_data: &mut IPData) {
    let ip = [
        ip_data.cf_connecting_ip.as_ref(),
        ip_data.x_real_ip.as_ref(),
        ip_data.x_forwarded_for.as_ref(),
    ];
    let ip = ip.iter().find(|ip| ip.is_some());
    if let Some(ip) = ip {
        let ip = ip.as_ref().unwrap();
        let response = get_ip_info(ip).await;
        match response {
            Ok(response) => {
                ip_data.ip_api_is_response = Some(response);
            }
            Err(e) => {
                tracing::error!("Error getting IP info: {:?}", e);
            }
        }
        for service in IP_GEO_LOCATE_SERVICES {
            let response = Locator::get(ip, service).await;
            match response {
                Ok(response) => {
                    ip_data.ip_geolocate_response = Some(LocatorDe::new(response));
                    break;
                }
                Err(e) => {
                    tracing::error!("Error getting IP info: {:?}", e);
                }
            }
        }
    }
}
