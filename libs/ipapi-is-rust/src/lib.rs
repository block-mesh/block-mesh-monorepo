mod response;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub async fn get_ip_info(ip: &str) -> Result<response::IpApiIsResponse, reqwest::Error> {
    let url = format!("https://api.ipapi.is?q={}", ip);
    let response = reqwest::get(&url)
        .await?
        .json::<response::IpApiIsResponse>()
        .await?;
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
