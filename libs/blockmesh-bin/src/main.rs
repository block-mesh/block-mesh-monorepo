mod app;
mod components;
mod leptos_state;
mod log;
mod page_routes;
mod pages;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
