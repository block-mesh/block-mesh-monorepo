use crate::leptos_state::LeptosTauriAppState;
use leptos::*;

#[component]
pub fn ConfigViewer() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let config_json = Signal::derive(move || {
        serde_json::to_string_pretty(&state.app_config.get()).unwrap_or_default()
    });

    view! { <pre>{move || config_json.get()}</pre> }
}
