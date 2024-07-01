#![allow(unused_variables, unused_imports)]
use crate::utils::ext_state::{AppState, AppStatus};
use leptos::*;
use leptos_dom::tracing;
use std::time::Duration;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let url = Signal::derive(move || state.blockmesh_url.get());
    let invite_code = Signal::derive(move || state.invite_code.get());
    let invite_url = Signal::derive(move || {
        format!(
            "{}/register?invite_code={}",
            state.blockmesh_url.get(),
            invite_code.get()
        )
    });

    let copy_to_clipboard = move |_: ()| {
        #[cfg(web_sys_unstable_apis)]
        {
            if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
                let invite_url_string = invite_url.get();
                tracing::info!("invite_url_string = {}", invite_code.get());
                if invite_code.get().is_empty() {
                    AppState::set_error("Missing invite code".to_string(), state.error);
                    return;
                }
                let _ = clipboard.write_text(&invite_url_string);
                AppState::set_success("Copied to clipboard".to_string(), state.success);
            } else {
                AppState::set_error("Failed to copy".to_string(), state.error);
            }
        }
        #[cfg(not(web_sys_unstable_apis))]
        {}
    };

    view! {
        <div class="auth-card">
            <img class="background-image" src="/assets/background.png" alt="background"/>
            <div class="auth-card-frame"></div>
            <div class="auth-card-top">
                <div class="auth-card-logout">
                    <svg
                        title="logout"
                        xmlns="http://www.w3.org/2000/svg"
                        height="24px"
                        viewBox="0 -960 960 960"
                        width="24px"
                        fill="#fab457cc"
                    >
                        <path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h280v80H200v560h280v80H200Zm440-160-55-58 102-102H360v-80h327L585-622l55-58 200 200-200 200Z"></path>
                    </svg>
                </div>
            </div>
            <div class="auth-card-body">
                <img class="logo" src="/assets/block-mesh-logo.png" alt="logo"/>
                <h1>BlockMesh</h1>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="auth-card-version">version: 0.0.27</small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>ohaddahan@gmail.com</strong>
                    </div>
                </div>
            </div>
            <div class="auth-card-bottom logged-bottom">
                <button class="auth-card-button">Open Dashboard</button>
                <button class="auth-card-button">Refer</button>
            </div>
        </div>
    }
}
