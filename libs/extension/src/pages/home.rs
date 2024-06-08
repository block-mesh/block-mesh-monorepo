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

    let copy_to_clipboard = move |_| {
        #[cfg(web_sys_unstable_apis)]
        {
            if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
                let invite_url_string = invite_url.get();
                tracing::info!("invite_url_string = {}", invite_code.get());
                if invite_code.get().is_empty() {
                    AppState::set_error("Missing invite code".to_string(), state.error.clone());
                    return;
                }
                let _ = clipboard.write_text(&invite_url_string);
                AppState::set_success("Copied to clipboard".to_string(), state.success.clone());
            } else {
                AppState::set_error("Failed to copy".to_string(), state.error.clone());
            }
        }
        #[cfg(not(web_sys_unstable_apis))]
        {}
    };

    view! {
        {move || match state.status.get() {
            AppStatus::LoggedIn => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center">
                        <div class="bg-gray-800 p-8 shadow-md w-80">
                            <button class="w-3/8 m-2 focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none">
                                <a href=url.get() target="_blank">
                                    Open Dashboard
                                </a>
                            </button>
                            <button
                                class="w-3/8 m-2 focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none"
                                on:click=copy_to_clipboard
                            >
                                Refer
                            </button>
                        </div>
                    </div>
                }
                    .into_view()
            }
            AppStatus::LoggedOut => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center text-center">
                        <div class="bg-gray-800 p-8 shadow-md w-80">
                            <p class="text-white">You are not logged in</p>
                        </div>
                    </div>
                }
                    .into_view()
            }
            AppStatus::WaitingEmailVerification => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center text-center">
                        <div class="bg-gray-800 p-8 shadow-md w-80">
                            <p class="text-white">Please verify your email address and login</p>
                        </div>
                    </div>
                }
                    .into_view()
            }
        }}
    }
}
