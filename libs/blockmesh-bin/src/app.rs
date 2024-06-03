use crate::components::navigation::Navigation;
use crate::leptos_state::LeptosTauriAppState;
use crate::page_routes::PageRoutes;
use crate::pages::dashboard::Dashboard;
use crate::pages::home::Home;
use crate::pages::settings_wrapper::SettingsWrapper;
use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::cli::CommandsEnum;
use leptos::*;
use leptos_router::{Route, Router, Routes};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(inline_js = r#"
        export async function invoke(cmd, args) {
            try {
                return await window.__TAURI__.tauri.invoke(cmd, args);
            } catch (e) {
                console.error(`Error in invoke ${cmd} : ${e}`);
                const t = typeof e;
                if (t === 'string') {
                    return { error: e };
                } else if (t === 'object') {
                    return { error: e?.message };
                } else {
                    return { error: 'Unknown error', e };
                }
            }
        }"#)]
extern "C" {
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn invoke_tauri(cmd: &str, args: JsValue) -> Result<JsValue, MyJsError> {
    let result = invoke(cmd, args).await;
    let error_attribute = JsValue::from_str("error");
    if let Ok(error) = js_sys::Reflect::get(&result, &error_attribute) {
        if error.is_string() {
            let error = error.as_string().unwrap();
            tracing::error!("Command: '{}' , Failed with error: '{}'", cmd, error);
            return Err(MyJsError {
                message: error,
                cmd: cmd.to_string(),
            });
        }
    }
    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyJsError {
    pub cmd: String,
    pub message: String,
}

impl Display for MyJsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command '{}': | Error: '{}'", self.cmd, self.message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetAppConfigArgs {
    pub config: AppConfig,
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(LeptosTauriAppState::default());
    let state = expect_context::<LeptosTauriAppState>();

    let (task_status, set_task_status) = create_signal(
        state
            .app_config
            .get_untracked()
            .task_status
            .unwrap_or_default()
            .to_string(),
    );

    let _resource = create_resource(
        move || {},
        |_| async move {
            let app_config_json = invoke_tauri("get_app_config", JsValue::NULL).await;
            let app_config_json = match app_config_json {
                Ok(app_config_json) => {
                    if app_config_json.is_null() {
                        return;
                    }
                    app_config_json
                }
                Err(_) => return,
            };
            let app_config_json = app_config_json.as_string().unwrap();
            let mut app_config: AppConfig = serde_json::from_str(&app_config_json).unwrap();
            if app_config.mode.is_none() {
                app_config.mode = Some(CommandsEnum::ClientNode);
            }
            tracing::info!("Loaded app_config: {:?}", app_config);
            let state = expect_context::<LeptosTauriAppState>();
            state.app_config.set(app_config);
        },
    );

    let _interval = set_interval_with_handle(
        move || {
            spawn_local(async move {
                let result = invoke_tauri("get_task_status", JsValue::NULL).await;
                if let Ok(result) = result {
                    let result = result.as_string().unwrap();
                    let task = TaskStatus::from(result);
                    set_task_status.set(task.to_string());
                    state.app_config.update(|config| {
                        config.task_status = Some(task);
                    });
                }
            });
        },
        Duration::from_secs(30),
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
                                        path=PageRoutes::Dashboard.path()
                                        view=move || {
                                            view! { <Dashboard task_status/> }
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
