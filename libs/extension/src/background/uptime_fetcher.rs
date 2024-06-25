use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{GetUserUptimeRequest, GetUserUptimeResponse};
use leptos::*;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::AppState;

#[wasm_bindgen]
pub async fn uptime_fetcher() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = AppState::default();
    app_state.init_with_storage().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }

    let base_url = app_state.blockmesh_url.get_untracked();
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let query = GetUserUptimeRequest { email, api_token };

    if let Ok(response) = reqwest::Client::new()
        .post(format!("{}/api/get_user_uptime", base_url))
        .query(&query)
        .send()
        .await
    {
        if let Ok(response) = response.json::<GetUserUptimeResponse>().await {
            let duration_seconds = response.duration_seconds.unwrap_or(0.0);
            AppState::store_uptime(duration_seconds).await;
            app_state.uptime.update(|v| *v = duration_seconds);
        }
    }
}
