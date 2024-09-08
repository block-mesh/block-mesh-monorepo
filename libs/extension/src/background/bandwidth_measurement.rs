use crate::utils::connectors::set_panic_hook;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use block_mesh_common::chrome_storage::AuthStatus;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{ReportBandwidthRequest, ReportBandwidthResponse};
use leptos::*;
use leptos_dom::tracing;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use speed_test::download::test_download;
use speed_test::latency::test_latency;
use speed_test::types::metadata::Metadata;
use speed_test::upload::test_upload;
use speed_test::utils::metadata::fetch_metadata;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn measure_bandwidth() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    let app_state = ExtensionWrapperState::default();
    app_state.init_with_storage().await;

    if !app_state.has_api_token() {
        return;
    }
    if app_state.status.get_untracked() == AuthStatus::LoggedOut {
        return;
    }
    let base_url = &app_state.blockmesh_url.get_untracked();
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();

    let (download_speed, upload_speed) =
        measure_bandwidth_inner(base_url, &email, &api_token).await;
    app_state.download_speed.set(download_speed);
    app_state.upload_speed.set(upload_speed);
}

pub async fn measure_bandwidth_inner(base_url: &str, email: &str, api_token: &Uuid) -> (f64, f64) {
    let download_speed = test_download(100_000).await.unwrap_or_default();
    let upload_speed = test_upload(100_000).await.unwrap_or_default();
    let latency = test_latency().await.unwrap_or_default();
    let metadata = fetch_metadata().await.unwrap_or_default();
    ExtensionWrapperState::store_download_speed(download_speed).await;
    ExtensionWrapperState::store_upload_speed(upload_speed).await;
    let _ = submit_bandwidth(
        base_url,
        email,
        api_token,
        download_speed,
        upload_speed,
        latency,
        metadata,
    )
    .await;
    (download_speed, upload_speed)
}

#[tracing::instrument(name = "submit_bandwidth", err)]
pub async fn submit_bandwidth(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
    download_speed: f64,
    upload_speed: f64,
    latency: f64,
    metadata: Metadata,
) -> anyhow::Result<ReportBandwidthResponse> {
    let body = ReportBandwidthRequest {
        email: email.to_string(),
        api_token: *api_token,
        download_speed,
        upload_speed,
        latency,
        city: metadata.city,
        country: metadata.country,
        ip: metadata.ip,
        asn: metadata.asn,
        colo: metadata.colo,
    };

    let response = reqwest::Client::new()
        .post(format!("{}/api/submit_bandwidth", base_url))
        .json(&body)
        .send()
        .await?;
    let response: ReportBandwidthResponse = response.json().await?;
    Ok(response)
}
