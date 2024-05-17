use crate::background::tasks::{get_task, run_task, submit_task};
use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::AppState;
use crate::utils::log::log;
use leptos::SignalGetUntracked;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn task_poller() {
    set_panic_hook();
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

    let task = match get_task(
        &app_state.blockmesh_url.get_untracked(),
        &app_state.email.get_untracked(),
        &app_state.api_token.get_untracked(),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            return;
        }
    };
    let finished_task = match run_task(&task.url, &task.method, task.headers, task.body).await {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
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
            log!("{e}");
        }
    };
}
