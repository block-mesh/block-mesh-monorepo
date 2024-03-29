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
mod tests {}
