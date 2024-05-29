use crate::background::tasks::{get_task, run_task, submit_task};
use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::{AppState, AppStatus};
use crate::utils::log::{log, setup_leptos_tracing};
use leptos::SignalGetUntracked;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn task_poller() {
    set_panic_hook();
    setup_leptos_tracing();
    // try to init the browser runtime, but there is nothing we can do if it's missing
    // if it does, there is either a bug or something changed in the browser implementation
    // The runtime is a global singleton. It can probably work with OnceCell or lazy_static!.
    // match get_runtime().await {
    //     Ok(v) => v,
    //     Err(e) => {
    //         log!("{e}");
    //         return;
    //     }
    // };

    let app_state = AppState::new().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        log!("has_api_token == false");
        return;
    }
    if app_state.status.get_untracked() == AppStatus::LoggedOut {
        log!("status == {}", app_state.status.get_untracked());
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
            log!("get_task error: {e}");
            return;
        }
    };
    let task = match task {
        Some(v) => v,
        None => {
            log!("no task found");
            return;
        }
    };
    let finished_task = match run_task(&task.url, &task.method, task.headers, task.body).await {
        Ok(v) => v,
        Err(e) => {
            log!("finished_task: {e}");
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
            log!("successfully submitted task");
        }
        Err(e) => {
            log!("submit_task: {e}");
        }
    };
}
