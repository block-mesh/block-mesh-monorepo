pub mod response;

#[tracing::instrument(name = "get_ip_info", ret, err)]
pub async fn get_ip_info(ip: &str) -> Result<response::IpApiIsResponse, reqwest::Error> {
    let url = format!("https://api.ipapi.is?q={}", ip);
    let response_result = reqwest::get(&url).await;
    let response = response_result.map_err(|e| {
        tracing::error!("Error getting IP info: {:?}", e);
        e
    })?;
    let response = response
        .json::<response::IpApiIsResponse>()
        .await
        .map_err(|e| {
            tracing::error!("Error deserializing IP info: {:?}", e);
            e
        })?;
    tracing::info!("IP info: {:?}", response);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_ip_info_test() {
        let ip = "107.174.138.172";
        let result = get_ip_info(ip).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.ip, ip.to_string());
        assert_eq!(result.rir, "ARIN".to_string());
        assert_eq!(result.is_bogon, false);
        assert_eq!(result.is_mobile, false);
        assert_eq!(result.is_crawler, false);
        assert_eq!(result.is_datacenter, true);
        assert_eq!(result.is_tor, true);
        assert_eq!(result.is_proxy, true);
        assert_eq!(result.is_vpn, false);
    }
}
