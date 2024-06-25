use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use leptos::*;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use wasm_bindgen::prelude::wasm_bindgen;

use speed_test::metadata::fetch_metadata;

use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::{AppState, AppStatus};

#[wasm_bindgen]
pub async fn report_uptime() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = AppState::default();
    app_state.init_with_storage().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }
    if app_state.status.get_untracked() == AppStatus::LoggedOut {
        return;
    }

    let base_url = app_state.blockmesh_url.get_untracked();
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();
    let metadata = fetch_metadata().await.unwrap_or_default();

    let query = ReportUptimeRequest {
        email,
        api_token,
        ip: if metadata.ip.is_empty() {
            None
        } else {
            Some(metadata.ip)
        },
    };

    if let Ok(response) = reqwest::Client::new()
        .post(format!("{}/api/report_uptime", base_url))
        .query(&query)
        .send()
        .await
    {
        let _ = response.json::<ReportUptimeResponse>().await;
    }
}
