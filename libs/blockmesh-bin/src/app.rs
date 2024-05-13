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
    // let (name, set_name) = create_signal(String::new());
    // let (greet_msg, set_greet_msg) = create_signal(String::new());
    let (cli_args, _set_cli_args) = create_signal(CliArgs::default());
    let _resource = create_resource(move || cli_args.get(), |_| async move {});

    // let update_name = move |ev: Event| {
    //     let v = event_target_value(&ev);
    //     set_name.set(v);
    // };
    // let greet = move |ev: SubmitEvent| {
    //     ev.prevent_default();
    //     spawn_local(async move {
    //         let name = name.get_untracked();
    //         if name.is_empty() {
    //             return;
    //         }
    //
    //         let args = to_value(&GreetArgs { name: &name }).unwrap();
    //         Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // let new_msg = invoke("greet", args).await.as_string().unwrap();
    // set_greet_msg.set(new_msg);
    // });
    // };

    view! {
        <div class="h-full bg-gray-900">
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
