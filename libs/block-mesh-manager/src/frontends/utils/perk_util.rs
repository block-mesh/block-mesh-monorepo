use block_mesh_common::interfaces::server_api::PerkResponse;
use leptos::window;
use reqwest::Client;

pub async fn sync_perk() -> anyhow::Result<PerkResponse> {
    let client = Client::new();
    let response: PerkResponse = client
        .get(format!("{}/api/intract_perk", window().origin()))
        .send()
        .await?
        .json()
        .await?;
    Ok(response)
}
