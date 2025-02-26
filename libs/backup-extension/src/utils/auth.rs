use anyhow::anyhow;
use block_mesh_common::interfaces::server_api::{
    GetTokenResponse, LoginForm, RegisterForm, RegisterResponse,
};
use block_mesh_common::routes_enum::RoutesEnum;

pub async fn login(
    blockmesh_url: &str,
    credentials: &LoginForm,
) -> anyhow::Result<GetTokenResponse> {
    let blockmesh_url = if blockmesh_url.contains("app") {
        blockmesh_url.replace("app", "api")
    } else {
        blockmesh_url.to_string()
    };
    let url = format!("{}/api{}", blockmesh_url, RoutesEnum::Api_GetToken);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&credentials)
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}

pub async fn register(blockmesh_url: &str, credentials: &RegisterForm) -> anyhow::Result<()> {
    let url = format!("{}{}", blockmesh_url, RoutesEnum::Static_UnAuth_RegisterApi);
    let client = reqwest::Client::new();
    let response = client.post(&url).form(credentials).send().await?;
    let response: RegisterResponse = response.json().await?;
    if response.status_code == 200 {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to register - {}",
            response.error.unwrap_or_default()
        ))
    }
}
