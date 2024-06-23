use crate::pages::page::Page;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::constants::{BLOCKMESH_VERSION, BLOCK_MESH_SUPPORT_EMAIL};
use leptos::*;
use std::time::Duration;

#[component]
pub fn NavBar(#[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let email = Signal::derive(move || state.email.get());
    let status = Signal::derive(move || state.status.get());
    let support_href = format!("mailto: {}", BLOCK_MESH_SUPPORT_EMAIL);
    let (download_speed, set_download_speed) = create_signal(state.download_speed.get_untracked());
    let (upload_speed, set_upload_speed) = create_signal(state.upload_speed.get_untracked());
    // let _display_bandwidth = Signal::derive(move || {
    //     let d = download_speed.get();
    //     let u = upload_speed.get();
    //     d > 0f64 && u > 0f64 && status.get() == AppStatus::LoggedIn
    // });
    set_timeout(
        move || {
            spawn_local(async move {
                let d = AppState::get_download_speed().await;
                let u = AppState::get_upload_speed().await;
                set_download_speed.set(d);
                set_upload_speed.set(u);
            })
        },
        Duration::from_millis(1000),
    );
    // set_interval(
    //     move || {
    //         spawn_local(async move {
    //             let d = AppState::get_download_speed().await;
    //             let u = AppState::get_upload_speed().await;
    //             set_download_speed.set(d);
    //             set_upload_speed.set(u);
    //         })
    //     },
    //     Duration::from_millis(60000),
    // );

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
                            <Show
                                when=move || status.get() == AppStatus::LoggedIn
                                fallback=|| {
                                    view! {}
                                }
                            >

                                <div class="mb-2">
                                    <video
                                        autoplay=true
                                        loop=true
                                        class="w-24 h-24"
                                        src="/assets/wifi-animation.webm"
                                    ></video>
                                </div>
                            </Show>

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
                            <Show
                                when=move || false
                                fallback=|| {
                                    view! {}
                                }
                            >

                                <div class="mb-2">
                                    <span class="mr-1 align-middle text-left">Upload:</span>
                                    <span class="align-middle text-right">
                                        {{ format!("{:.2} [mbit/s]", upload_speed.get()) }}
                                    </span>
                                </div>
                                <div class="mb-2">
                                    <span class="mr-1 align-middle text-left">Download:</span>
                                    <span class="align-middle text-right">
                                        {{ format!("{:.2} [mbit/s]", download_speed.get()) }}
                                    </span>
                                </div>
                            </Show>

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
