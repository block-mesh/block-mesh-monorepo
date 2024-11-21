use crate::frontends::context::extension_state::ExtensionContext;
use block_mesh_common::chrome_storage::AuthStatus;
use leptos::logging::log;
use leptos::*;
use leptos_router::use_navigate;
use leptos_use::use_debounce_fn;

#[component]
pub fn ExtensionNavigator() -> impl IntoView {
    let state = expect_context::<ExtensionContext>();
    let navigate = use_navigate();
    let debounce = use_debounce_fn(
        move || {
            log!("debounce {}", state.status.get());
            match state.status.get() {
                AuthStatus::LoggedIn => {
                    navigate("/ext/logged_in", Default::default());
                }
                AuthStatus::Registering => {
                    navigate("/ext/register", Default::default());
                }
                AuthStatus::LoggedOut => {
                    navigate("/ext/login", Default::default());
                }
                AuthStatus::WaitingEmailVerification => {
                    navigate("/ext/login", Default::default());
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
