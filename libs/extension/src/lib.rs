mod utils;
use wasm_bindgen::prelude::*;

use crate::utils::get_runtime::get_runtime;
use crate::utils::log::log;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Makes JS `console.log` available in Rust
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

/// The main entry point callable from `background.js`.
#[wasm_bindgen]
pub async fn task_poller() {
    set_panic_hook();
    // try to init the browser runtime, but there is nothing we can do if it's missing
    // if it does, there is either a bug or something changed in the browser implementation
    // The runtime is a global singleton. It can probably work with OnceCell or lazy_static!.
    match get_runtime().await {
        Ok(v) => v,
        Err(e) => {
            log!("{e}");
            return;
        }
    };

    let _ = reqwest::Client::new()
        .get("https://mocki.io/v1/17e60da5-ab9d-4501-99f2-4f12f464a6e8")
        .send()
        .await
        .map(|v| log!("{:?}", v))
        .map_err(|e| log!("{e}"));
}

/// This is a proxy for report_progress() in progress.js
/// to send messages to other js scripts.
#[wasm_bindgen(inline_js = r#"
    export function report_progress(msg) {
        function onSuccess(message) {
            console.log(`report_progress::onSuccess: ${JSON.stringify(message)}`);
        }
        function onError(error) {
            console.log(`report_progress::onError: ${error}`);
        }
        try {
            chrome.runtime.sendMessage(msg).then(onSuccess, onError)
        } catch (e) {
            console.log('report_progress', { e })
        }
    }"#)]
extern "C" {
    pub fn report_progress(msg: &str);
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}
