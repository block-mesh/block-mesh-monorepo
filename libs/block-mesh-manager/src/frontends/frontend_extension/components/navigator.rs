use crate::frontends::frontend_extension::extension_state::ExtensionState;
use block_mesh_common::chrome_storage::AuthStatus;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn ExtensionNavigator() -> impl IntoView {
    let state = expect_context::<ExtensionState>();
    let navigate = use_navigate();
    create_effect(move |_| match state.status.get() {
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
    });

    view! {}
}
