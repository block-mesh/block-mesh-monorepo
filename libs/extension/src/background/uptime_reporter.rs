use crate::background::operation_mode::OperationMode;
use crate::utils::connectors::set_panic_hook;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use block_mesh_common::chrome_storage::AuthStatus;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use speed_test::metadata::fetch_metadata;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn report_uptime() {
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

    let base_url = app_state.blockmesh_url.get_untracked();
    let email = app_state.email.get_untracked();
    let api_token = app_state.api_token.get_untracked();

    report_uptime_inner(&base_url, &email, &api_token, OperationMode::Http).await;
}

pub async fn report_uptime_inner(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
    operation_mode: OperationMode,
) -> Option<ReportUptimeRequest> {
    let metadata = fetch_metadata().await.unwrap_or_default();
    let ip = if metadata.ip.is_empty() {
        None
    } else {
        Some(metadata.ip)
    };

    let query = ReportUptimeRequest {
        email: email.to_string(),
        api_token: *api_token,
        ip: ip.clone(),
    };

    match operation_mode {
        OperationMode::Http => {
            if let Ok(response) = reqwest::Client::new()
                .post(format!(
                    "{}/{}/api{}",
                    base_url,
                    DeviceType::Extension,
                    RoutesEnum::Api_ReportUptime
                ))
                .query(&query)
                .send()
                .await
            {
                let _ = response.json::<ReportUptimeResponse>().await;
            }
            None
        }
        OperationMode::WebSocket => Some(ReportUptimeRequest {
            email: email.to_string(),
            api_token: *api_token,
            ip: ip.clone(),
        }),
    }
}
