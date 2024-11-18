use leptos::logging::log;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::window;

#[wasm_bindgen]
pub fn hello_world() {
    log!("Hello World!");
    if let Some(w) = window() {
        log!(
            "Found window , origin: {}",
            w.location().origin().unwrap_or_default()
        );
    }
}
