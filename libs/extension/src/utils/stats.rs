use block_mesh_common::interfaces::server_api::{GetStatsRequest, GetStatsResponse};

#[allow(dead_code)]
pub async fn get_stats(
    blockmesh_url: &str,
    credentials: &GetStatsRequest,
) -> anyhow::Result<GetStatsResponse> {
    let url = format!("{}/api/get_stats", blockmesh_url);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(credentials)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
