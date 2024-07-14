use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

#[allow(unused_imports)]
use background::*;
use block_mesh_common::constants::DeviceType;
use logger_leptos::leptos_tracing::setup_leptos_tracing;

use crate::pages::options::ExtensionOptionsPage;
use crate::pages::popup::ExtensionPopupPage;
use crate::utils::connectors::set_panic_hook;
#[allow(unused_imports)]
use pages::*;

mod background;
mod components;
mod pages;
mod utils;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn mount_popup() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    mount_to(
        document()
            .get_element_by_id("mount_to")
            .unwrap()
            .unchecked_into(),
        ExtensionPopupPage,
    );
}

#[wasm_bindgen]
pub fn mount_options() {
    set_panic_hook();
    setup_leptos_tracing(None, DeviceType::Extension);
    mount_to_body(ExtensionOptionsPage);
}
