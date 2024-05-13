use crate::components::navigation::Navigation;
use crate::page_routes::PageRoutes;
use crate::pages::settings_wrapper::SettingsWrapper;
use crate::state::LeptosTauriAppState;
use block_mesh_common::cli::CliArgs;
// use leptos::ev::Event;
// use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use leptos_router::{Route, Router, Routes};
use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
use crate::log::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(LeptosTauriAppState::default());
    let (cli_args, _set_cli_args) = create_signal(CliArgs::default());
    let _resource = create_resource(
        move || cli_args.get(),
        |_| async move {
            log!("Here");
        },
    );

    view! {
        <div class="h-screen bg-gray-800">
            <div class="h-full">
                <Router>
                    <Navigation/>
                    <div class="lg:pl-72">
                        <main class="py-10">
                            <div class="px-4 sm:px-6 lg:px-8">
                                <Routes>
                                    <Route
                                        path=PageRoutes::Home.path()
                                        view=move || {
                                            view! { <div>Home</div> }
                                        }
                                    />

                                    <Route
                                        path=PageRoutes::Settings.path()
                                        view=move || {
                                            view! { <SettingsWrapper/> }
                                        }
                                    />

                                </Routes>
                            </div>
                        </main>
                    </div>
                </Router>
            </div>
        </div>
    }
}
