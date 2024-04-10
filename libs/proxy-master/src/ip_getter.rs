use anyhow::anyhow;
use block_mesh_common::constants::BLOCK_MESH_IP_WORKER;
use solana_client::client_error::reqwest;
use std::net::IpAddr;
use std::str::FromStr;

pub async fn get_ip() -> anyhow::Result<IpAddr> {
    let local_address = IpAddr::from_str("0.0.0.0").unwrap();
    let client = reqwest::Client::builder()
        .local_address(local_address)
        .build()
        .unwrap();

    let json: serde_json::Value = client
        .get(BLOCK_MESH_IP_WORKER)
        .send()
        .await?
        .json()
        .await?;

    let cf_connecting_ip = json.get("cf_connecting_ip");
    match cf_connecting_ip {
        None => (),
        Some(ip) => {
            let ip = IpAddr::from_str(ip.as_str().unwrap())?;
            return Ok(ip);
        }
    }

    Err(anyhow!("No IP found"))
}
