use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

use app::*;

mod app;
mod app_router;
mod components;
mod leptos_state;
mod log;
mod page_routes;
mod pages;
mod tauri_connector;

#[wasm_bindgen(start)]
pub fn mount_tauri_leptos() {
    console_error_panic_hook::set_once();
    // setup_leptos_tracing(None, DeviceType::Desktop);
    mount_to(
        document()
            .get_element_by_id("mount_to")
            .unwrap()
            .unchecked_into(),
        App,
    );
}
