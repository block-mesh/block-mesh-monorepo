use crate::frontends::context::auth_context::AuthContext;
use block_mesh_common::chrome_storage::AuthStatus;
use leptos::logging::log;
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn TauriNavigator() -> impl IntoView {
    let state = expect_context::<AuthContext>();
    let navigate = use_navigate();
    create_effect(move |_| {
        let status = state.status.get();
        log!("TauriNavigator {}", status);
        match status {
            AuthStatus::LoggedIn => {
                navigate("/tauri/logged_in", Default::default());
            }
            AuthStatus::Registering => {
                navigate("/tauri/register", Default::default());
            }
            AuthStatus::LoggedOut => {
                navigate("/tauri/login", Default::default());
            }
            AuthStatus::WaitingEmailVerification => {
                navigate("/tauri/login", Default::default());
            }
            AuthStatus::UpdateVersion => {
                navigate("/tauri/update_version", Default::default());
            }
        }
    });

    view! {}
}
