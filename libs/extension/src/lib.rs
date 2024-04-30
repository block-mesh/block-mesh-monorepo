use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(unused_imports)]
use background::*;

#[allow(unused_imports)]
use pages::*;

use crate::pages::options::Options;
use crate::pages::popup::Popup;

use crate::utils::connectors::set_panic_hook;

mod background;
mod components;
mod pages;
mod utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn mount_popup() {
    set_panic_hook();
    mount_to_body(Popup);
}

#[wasm_bindgen]
pub fn mount_options() {
    set_panic_hook();
    mount_to_body(Options);
}
