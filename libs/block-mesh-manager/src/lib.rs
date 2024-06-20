use cfg_if::cfg_if;
pub mod frontend;

cfg_if! { if #[cfg(feature = "ssr")] {
    pub mod configuration;
    pub mod database;
    pub mod domain;
    pub mod envars;
    pub mod errors;
    pub mod emails;
    pub mod middlewares;
    pub mod notification;
    pub mod routes;
    pub mod startup;
    pub mod telemetry;
    pub mod worker;
}}

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::frontend::app::App;
    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();
        mount_to_body(App);
    }
}}
