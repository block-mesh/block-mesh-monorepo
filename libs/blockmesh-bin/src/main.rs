mod app;
mod components;
mod log;
mod page_routes;
mod pages;
mod state;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
