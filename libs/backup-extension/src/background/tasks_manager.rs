use crate::background::tasks::{get_task, run_task, submit_task};
use crate::utils::connectors::set_panic_hook;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use block_mesh_common::chrome_storage::AuthStatus;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::GetTaskResponse;
use chrono::Utc;
use leptos::SignalGetUntracked;
use leptos::*;
use leptos_dom::tracing;
use logger_leptos::leptos_tracing::setup_leptos_tracing;
use speed_test::metadata::fetch_metadata;
use std::cmp;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn task_poller() {
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

    let task = match get_task(&base_url, &email, &api_token).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("get_task error: {e}");
            return;
        }
    };

    let task = match task {
        Some(v) => v,
        None => {
            return;
        }
    };

    task_poller_inner(&base_url, &email, &api_token, &task).await;
}

pub async fn task_poller_inner(
    base_url: &str,
    email: &str,
    api_token: &Uuid,
    task: &GetTaskResponse,
) {
    let metadata = fetch_metadata().await.unwrap_or_default();
    let start = Utc::now();

    let finished_task = match run_task(
        &task.url,
        &task.method,
        task.headers.clone(),
        task.body.clone(),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("finished_task: error: {e}");
            let end = Utc::now();
            let response_time = cmp::max((end - start).num_milliseconds(), 1) as f64;
            match submit_task(
                base_url,
                email,
                api_token,
                &task.id,
                520,
                "".to_string(),
                &metadata,
                response_time,
            )
            .await
            {
                Ok(_) => {
                    tracing::info!("successfully submitted failed task");
                }
                Err(e) => {
                    tracing::error!("submit_task: error: {e}");
                }
            }
            return;
        }
    };
    let end = Utc::now();
    let response_time = cmp::max((end - start).num_milliseconds(), 1) as f64;

    match submit_task(
        base_url,
        email,
        api_token,
        &task.id,
        finished_task.status,
        finished_task.raw,
        &metadata,
        response_time,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("successfully submitted task");
        }
        Err(e) => {
            tracing::error!("submit_task: error: {e}");
        }
    };
}
