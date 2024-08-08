use block_mesh_solana_client::manager::FullRouteHeader;
use reqwest::ClientBuilder;
use solana_client::client_error::reqwest;
use std::time::Duration;

#[tracing::instrument(name = "register_token", skip(solana_manager_header))]
pub async fn register_token(
    proxy_url: &str,
    solana_manager_header: &FullRouteHeader,
) -> anyhow::Result<()> {
    let response = ClientBuilder::new()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap_or_default()
        .post(proxy_url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(solana_manager_header)?)
        .send()
        .await?;
    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => {
            let msg = format!("register_token::Error: {:?}", response.text().await?);
            tracing::error!(msg);
            return Err(anyhow::anyhow!(msg));
        }
    }
}
