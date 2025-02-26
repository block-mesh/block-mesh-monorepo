use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::CheckTokenRequest;
use block_mesh_common::routes_enum::RoutesEnum;

#[allow(dead_code)]
pub async fn check_token(
    blockmesh_url: &str,
    credentials: &CheckTokenRequest,
) -> anyhow::Result<()> {
    let blockmesh_url = if blockmesh_url.contains("app") {
        blockmesh_url.replace("app", "api")
    } else {
        blockmesh_url.to_string()
    };
    let url = format!(
        "{}/{}/api{}",
        blockmesh_url,
        DeviceType::Extension,
        RoutesEnum::Api_CheckToken
    );
    let client = reqwest::Client::new();
    let r = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&credentials)
        .send()
        .await?;
    let _: serde_json::Value = r.json().await?;
    Ok(())
}
