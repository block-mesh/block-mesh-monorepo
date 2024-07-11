use crate::utils::connectors::send_to_iframe;
use crate::utils::ext_state::AppState;
use crate::utils::log::log;
use leptos::*;

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    let state = AppState::init_resource(state);

    let logout = create_action(move |_: &()| async move {
        match state.get() {
            None => (),
            Some(s) => s.clear().await,
        };
    });

    let _on_logout = move |_: ()| {
        log!("Logout");
        logout.dispatch(());
    };

    let click = create_action(|_| async move {
        log!("here");
        if let Ok(js_args) = serde_wasm_bindgen::to_value("http://localhost:8000") {
            send_to_iframe("blockmesh_url", js_args).await;
        }
    });

    view! { <button on:click=move |_| click.dispatch(())>Send shit</button> }
}
