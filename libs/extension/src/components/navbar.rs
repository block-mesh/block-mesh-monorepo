use crate::pages::page::Page;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::constants::{BLOCKMESH_VERSION, BLOCK_MESH_SUPPORT_EMAIL};
use leptos::*;

#[component]
pub fn NavBar(#[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let email = Signal::derive(move || state.email.get());
    let status = Signal::derive(move || state.status.get());
    let support_href = format!("mailto: {}", BLOCK_MESH_SUPPORT_EMAIL);

    view! {
        <nav>
            <div class="flex items-center justify-center h-full">
                <div class="w-80 bg-gray-800 p-8 shadow-md">
                    <div class="flex justify-center">
                        <div class="mt-4 flex flex-col justify-center text-center text-white">
                            <div class="mb-2">
                                <a href="https://app.blockmesh.xyz" target="_blank">
                                    <img
                                        class="w-24 h-24"
                                        src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                                    />
                                </a>
                            </div>
                            <div class="mb-2">
                                <span class="mr-1 align-middle">Version:</span>
                                <span class="align-middle">{{ BLOCKMESH_VERSION }}</span>
                                <a href=support_href target="_blank" class="align-middle">
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
                                </a>
                            </div>

                        </div>
                    </div>
                    <div class="flex justify-center mt-4">
                        <Show
                            when=move || status.get() == AppStatus::LoggedIn
                            fallback=|| {
                                view! {
                                    <a
                                        href=Page::Login.path()
                                        class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                    >
                                        "Login"
                                    </a>
                                    <a
                                        href=Page::Register.path()
                                        class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                    >
                                        "Register"
                                    </a>
                                }
                            }
                        >

                            <div class="flex justify-center text-center mt-4 text-white flex-col">
                                <div class="mb-2">{move || email.get().to_string()}</div>
                                <div class="mb-2">
                                    <a
                                        href="#"
                                        class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                        on:click=move |_| on_logout.call(())
                                    >
                                        "Logout"
                                    </a>
                                </div>
                            </div>
                        </Show>

                    </div>
                </div>
            </div>
        </nav>
    }
}
