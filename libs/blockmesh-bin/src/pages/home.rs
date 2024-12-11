use crate::leptos_state::LeptosTauriAppState;
use crate::pages::login::Login;
use crate::tauri_connector::connector::invoke_tauri;
use block_mesh_common::constants::{BLOCKMESH_VERSION, BLOCK_MESH_SUPPORT_EMAIL};
use leptos::*;
use leptos_router::A;
use wasm_bindgen::JsValue;

#[component]
pub fn Home() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let email = Signal::derive(move || state.app_config.get().email);
    let support_href = move || format!("mailto: {}", BLOCK_MESH_SUPPORT_EMAIL);
    let logout = move || {
        spawn_local(async move {
            let state = expect_context::<LeptosTauriAppState>();
            if invoke_tauri("logout", JsValue::NULL).await.is_ok() {
                LeptosTauriAppState::check_token(&state).await;
            }
        });
    };
    view! {
        <div>
            <Show
                fallback=move || {
                    view! { <Login/> }
                }

                when=move || state.logged_in.get()
            >
                <div class="flex items-center justify-center h-full">
                    <div class="w-80 bg-gray-800 p-8 shadow-md">
                        <div class="flex justify-center">
                            <div class="mt-4 flex flex-col justify-center text-center text-white">
                                <div class="mb-2">
                                    <A href="https://app.blockmesh.xyz" target="_blank">
                                        <img
                                            class="w-24 h-24"
                                            src="https://r2-images.blockmesh.xyz/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/.png"
                                        />
                                    </A>
                                </div>
                                <div class="mb-2">
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke-width="1.5"
                                        stroke="currentColor"
                                        class="size-6"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            d="M8.288 15.038a5.25 5.25 0 0 1 7.424 0M5.106 11.856c3.807-3.808 9.98-3.808 13.788 0M1.924 8.674c5.565-5.565 14.587-5.565 20.152 0M12.53 18.22l-.53.53-.53-.53a.75.75 0 0 1 1.06 0Z"
                                        ></path>
                                    </svg>

                                </div>

                                <div class="mb-2">
                                    <span class="mr-1 align-middle">Version:</span>
                                    <span class="align-middle">{{ BLOCKMESH_VERSION }}</span>
                                    <A href=support_href target="_blank" class="align-middle">
                                        <svg
                                            class="ml-1 inline-block w-3 h-3 align-middle"
                                            aria-hidden="true"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke="currentColor"
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M10 11h2v5m-2 0h4m-2.592-8.5h.01M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
                                            ></path>
                                        </svg>
                                    </A>
                                </div>
                                <div class="flex justify-center text-center mt-4 text-white flex-col">
                                    <div class="mb-2">
                                        {move || { email.get().unwrap_or_default() }}
                                    </div>
                                    <div class="mb-2">
                                        <a
                                            href="#"
                                            class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                            on:click=move |_| {
                                                logout();
                                            }
                                        >

                                            "Logout"
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
