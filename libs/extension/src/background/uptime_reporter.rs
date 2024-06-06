use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use block_mesh_common::leptos_tracing::setup_leptos_tracing;

use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::AppState;

#[wasm_bindgen]
pub async fn report_uptime() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = AppState::new().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }

    let base_url = app_state.blockmesh_url.get_untracked();
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let query = ReportUptimeRequest { email, api_token };

    if let Ok(response) = reqwest::Client::new()
        .post(format!("{}/api/report_uptime", base_url))
        .query(&query)
        .send()
        .await
    {
        let _ = response.json::<ReportUptimeResponse>().await;
    }
}
