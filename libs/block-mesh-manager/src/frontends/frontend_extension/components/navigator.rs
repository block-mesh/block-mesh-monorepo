use crate::frontends::frontend_extension::extension_state::ExtensionState;
use block_mesh_common::chrome_storage::ExtensionStatus;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn ExtensionNavigator() -> impl IntoView {
    let state = expect_context::<ExtensionState>();
    let navigate = use_navigate();
    create_effect(move |_| match state.status.get() {
        ExtensionStatus::LoggedIn => {
            navigate("/ext/logged_in", Default::default());
        }
        ExtensionStatus::Registering => {
            navigate("/ext/register", Default::default());
        }
        ExtensionStatus::LoggedOut => {
            navigate("/ext/login", Default::default());
        }
        ExtensionStatus::WaitingEmailVerification => {
            navigate("/ext/login", Default::default());
        }
    });

    view! {}
}
