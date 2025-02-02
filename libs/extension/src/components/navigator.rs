use crate::utils::extension_wrapper_state::{ExtensionWrapperState, Page};
use block_mesh_common::chrome_storage::AuthStatus;
use leptos::logging::log;
use leptos::*;
use leptos_use::use_debounce_fn;

#[component]
pub fn ExtensionNavigator() -> impl IntoView {
    let state = expect_context::<ExtensionWrapperState>();
    let debounce = use_debounce_fn(
        move || {
            log!("debounce {}", state.status.get());
            match state.status.get() {
                AuthStatus::LoggedIn => {
                    state.page.update(|v| *v = Page::LoggedIn);
                }
                AuthStatus::Registering => {
                    state.page.update(|v| *v = Page::Register);
                }
                AuthStatus::LoggedOut => {
                    state.page.update(|v| *v = Page::Login);
                }
                AuthStatus::WaitingEmailVerification => {
                    state.page.update(|v| *v = Page::Login);
                }
            }
        },
        500.0,
    );
    create_effect(move |_| {
        log!("create_effect {}", state.status.get());
        debounce()
    });

    view! {}
}
