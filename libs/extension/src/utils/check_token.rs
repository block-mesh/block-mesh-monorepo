use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::CheckTokenRequest;
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::logging::log;

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
    log!("check_token - url = {}", url,);
    let r = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&credentials)
        .send()
        .await?;
    log!("check_token - response = {:#?}", r);
    let j: serde_json::Value = r.json().await?;
    log!("check_token - json = {:#?}", j);
    Ok(())
}
