use crate::components::navigation::Navigation;
use crate::leptos_state::LeptosTauriAppState;
use crate::log::log;
use crate::page_routes::PageRoutes;
use crate::pages::home::Home;
use crate::pages::settings_wrapper::SettingsWrapper;
use block_mesh_common::app_config::AppConfig;
use block_mesh_common::cli::CommandsEnum;
use leptos::*;
use leptos_router::{Route, Router, Routes};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
        export async function invoke(cmd, args) {
            try {
                return await window.__TAURI__.tauri.invoke(cmd, args);
            } catch (e) {
                console.error(e);
                return null;
            }
        }"#)]
extern "C" {
    // #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAppConfigArgs {
    pub config: AppConfig,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(LeptosTauriAppState::default());
    let _resource = create_resource(
        move || {},
        |_| async move {
            let app_config_json: JsValue = invoke("get_app_config", JsValue::NULL).await;
            if app_config_json.is_null() {
                return;
            }
            let app_config_json = app_config_json.as_string().unwrap();
            let mut app_config: AppConfig = serde_json::from_str(&app_config_json).unwrap();
            if app_config.mode.is_none() {
                app_config.mode = Some(CommandsEnum::ClientNode);
            }
            log!("Loaded app_config: {:?}", app_config);
            let state = expect_context::<LeptosTauriAppState>();
            state.app_config.set(app_config);
        },
    );

    view! {
        <div class="h-screen bg-gray-800">
            <div class="h-full">
                <Router>
                    <Navigation/>
                    <div class="lg:pl-72">
                        <main>
                            <div class="px-4 sm:px-6 lg:px-8">
                                <Routes>
                                    <Route
                                        path=PageRoutes::Home.path()
                                        view=move || {
                                            view! { <Home/> }
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
                <footer class="bg-gray-800 text-white py-6 border-t-2 border-white">
                    <div class="w-full flex flex-col items-center md:flex-row md:justify-between px-4">
                        <div class="text-center md:text-left"></div>
                        <div class="mt-4 md:mt-0">
                            <h5 class="text-gray-400 hover:text-white mx-2">BlockMesh Network</h5>
                            <a
                                href="https://x.com/blockmesh_xyz"
                                target="_blank"
                                class="text-gray-400 hover:text-white mx-2"
                            >
                                Contact Us
                            </a>
                        </div>
                    </div>
                </footer>
            </div>
        </div>
    }
}
