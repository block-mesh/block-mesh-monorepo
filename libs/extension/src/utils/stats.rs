use block_mesh_common::interface::{GetStatsRequest, GetStatsResponse};

pub async fn _get_stats(
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
