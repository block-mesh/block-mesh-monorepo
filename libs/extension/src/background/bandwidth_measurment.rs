use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::leptos_tracing::setup_leptos_tracing;
// use cfspeedtest::speedtest::test_download;
// use cfspeedtest::OutputFormat;
use leptos::*;
use speed_test::download::test_download;
use speed_test::latency::test_latency;
use speed_test::metadata::fetch_metadata;
use speed_test::upload::test_upload;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn measure_bandwidth() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = AppState::new().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }
    if app_state.status.get_untracked() == AppStatus::LoggedOut {
        return;
    }

    let _download_speed = test_download(100_000).await;
    let _upload_speed = test_upload(100_000).await;
    let _latency_speed = test_latency().await;
    let _metadata = fetch_metadata().await;
}
