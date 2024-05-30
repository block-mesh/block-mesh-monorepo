use crate::background::tasks::{get_task, run_task, submit_task};
use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::leptos_tracing::setup_leptos_tracing;
use leptos::SignalGetUntracked;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn task_poller() {
    set_panic_hook();
    setup_leptos_tracing();

    let app_state = AppState::new().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }
    if app_state.status.get_untracked() == AppStatus::LoggedOut {
        return;
    }

    let task = match get_task(
        &app_state.blockmesh_url.get_untracked(),
        &app_state.email.get_untracked(),
        &app_state.api_token.get_untracked(),
    )
    .await
    {
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
    let finished_task = match run_task(&task.url, &task.method, task.headers, task.body).await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("finished_task: {e}");
            return;
        }
    };

    match submit_task(
        &app_state.blockmesh_url.get_untracked(),
        &app_state.email.get_untracked(),
        &app_state.api_token.get_untracked(),
        &task.id,
        finished_task.status,
        finished_task.raw,
    )
    .await
    {
        Ok(_) => {
            tracing::info!("successfully submitted task");
        }
        Err(e) => {
            tracing::error!("submit_task: {e}");
        }
    };
}
