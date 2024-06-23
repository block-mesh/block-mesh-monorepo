mod app;
mod components;
mod leptos_state;
mod log;
mod page_routes;
mod pages;

use app::*;
use block_mesh_common::constants::DeviceType;
use leptos::*;
use logger_leptos::leptos_tracing::setup_leptos_tracing;

fn main() {
    console_error_panic_hook::set_once();
    setup_leptos_tracing(None, DeviceType::Desktop);
    mount_to_body(|| {
        view! { <App/> }
    })
}
