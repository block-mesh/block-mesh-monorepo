use block_mesh_solana_client::manager::SolanaManagerAuth;
use solana_client::client_error::reqwest;

#[tracing::instrument(name = "register_token", skip(solana_manager_header))]
pub async fn register_token(
    proxy_url: &str,
    solana_manager_header: &SolanaManagerAuth,
) -> anyhow::Result<()> {
    let response = reqwest::Client::new()
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
