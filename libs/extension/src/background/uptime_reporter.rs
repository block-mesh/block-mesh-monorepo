use crate::utils::connectors::set_panic_hook;
use crate::utils::ext_state::AppState;
use crate::utils::log::log;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn report_uptime() {
    set_panic_hook();
    let app_state = AppState::new().await;
    AppState::init(app_state).await;

    if !app_state.has_api_token() {
        return;
    }
    log!("here")
}
