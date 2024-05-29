#![allow(dead_code)]
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, WorkerGlobalScope};

/// Contains the right type of the browser runtime for the current browser
pub(crate) enum BrowserRuntime {
    ChromeWorker(WorkerGlobalScope),
    FireFoxWindow(Window),
}

/// Returns the right type of runtime for the current browser because
/// Firefox and Chrome do not agree on the parent object for Runtime in WebWorkers.
/// Firefox uses Window and Chrome uses WorkerGlobalScope.
pub async fn get_runtime() -> Result<BrowserRuntime, &'static str> {
    tracing::info!("Getting runtime");
    // try for chrome first and return if found
    // it should also work if FF switches to using WorkerGlobalScope as they should
    match js_sys::global().dyn_into::<WorkerGlobalScope>() {
        Ok(v) => {
            return Ok(BrowserRuntime::ChromeWorker(v));
        }
        Err(_) => {
            tracing::error!("ServiceWorkerGlobalScope unavailable");
        }
    };
    // this is a fallback for Firefox, but it does not make sense why they would use Window in
    // web workers
    match web_sys::window() {
        Some(v) => {
            return Ok(BrowserRuntime::FireFoxWindow(v));
        }
        None => {
            tracing::error!("Window unavailable");
        }
    };
    // no runtime was found, which is a serious problem
    // because all fetch calls require it
    // TODO: may be worth a retry
    Err("Missing browser runtime. It's a bug.")
}
