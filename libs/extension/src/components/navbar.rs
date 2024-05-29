use crate::pages::page::Page;
use crate::utils::ext_state::{AppState, AppStatus};
use leptos::*;

#[component]
pub fn NavBar(#[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let url = move || state.blockmesh_url.get();
    view! {
        <nav>
            <div class="flex items-center justify-center h-full">
                <div class="w-80 rounded-lg border-2 border-white bg-gray-800 p-8 shadow-md">
                    <div class="flex justify-center">
                        <a href="https://app.blockmesh.xyz" target="_blank">
                            <img
                                class="w-24 h-24"
                                src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                            />
                        </a>
                    </div>
                    <div class="flex justify-center mt-4 text-white">
                        <div class="mr-2 text-bold">URL:</div>
                        <a href={url} target="_blank" class="text-blue-500 hover:text-blue-800">
                            {url}
                        </a>
                    </div>
                    <div class="flex justify-center mt-4">
                        <Show
                            when=move || state.status.get() == AppStatus::LoggedIn
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

                            <a
                                href="#"
                                class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                                on:click=move |_| on_logout.call(())
                            >
                                "Logout"
                            </a>
                        </Show>
                    </div>
                </div>
            </div>
        </nav>
    }
}
