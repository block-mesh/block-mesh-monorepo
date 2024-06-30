use crate::app_router::AppRouter;
use crate::components::app_footer::AppFooter;
use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::invoke_tauri;
use block_mesh_common::app_config::{AppConfig, TaskStatus};
use block_mesh_common::cli::CommandsEnum;
use block_mesh_common::interfaces::server_api::GetTokenResponse;
use leptos::*;
use std::time::Duration;
use wasm_bindgen::JsValue;

#[component]
pub fn App() -> impl IntoView {
    provide_context(LeptosTauriAppState::default());
    let state = expect_context::<LeptosTauriAppState>();

    // let (task_status, set_task_status) = create_signal(
    //     state
    //         .app_config
    //         .get_untracked()
    //         .task_status
    //         .unwrap_or_default()
    //         .to_string(),
    // );

    let resource = create_resource(
        move || {
            state.app_config.get().email.is_some() || state.app_config.get().api_token.is_some()
        },
        |_| async move {
            tracing::info!("here 29");
            let app_config_json = invoke_tauri("get_app_config", JsValue::NULL).await;
            tracing::info!("here 31");
            let app_config_json = match app_config_json {
                Ok(app_config_json) => {
                    if app_config_json.is_null() {
                        tracing::info!("here 34");
                        return;
                    }
                    tracing::info!("here 37");
                    app_config_json
                }
                Err(e) => {
                    tracing::error!("error: {}", e);
                    return;
                }
            };
            tracing::info!("here 40");
            let app_config_json = app_config_json.as_string().unwrap();
            let mut app_config: AppConfig = serde_json::from_str(&app_config_json).unwrap();
            if app_config.mode.is_none() {
                app_config.mode = Some(CommandsEnum::ClientNode);
            }
            tracing::info!("Loaded app_config: {:?}", app_config);
            let state = expect_context::<LeptosTauriAppState>();
            state.app_config.set(app_config);

            if state.app_config.get_untracked().email.is_some()
                && state.app_config.get_untracked().api_token.is_some()
            {
                if let Ok(result) = invoke_tauri("check_token", JsValue::NULL).await {
                    if serde_wasm_bindgen::from_value::<GetTokenResponse>(result).is_ok() {
                        state.logged_in.update(|v| *v = true);
                    }
                }
            }
        },
    );

    let _interval = set_interval_with_handle(
        move || {
            spawn_local(async move {
                let result = invoke_tauri("get_task_status", JsValue::NULL).await;
                if let Ok(result) = result {
                    let result = result.as_string().unwrap();
                    let task = TaskStatus::from(result);
                    // set_task_status.set(task.to_string());
                    state.app_config.update(|config| {
                        config.task_status = Some(task);
                    });
                }

                let result = invoke_tauri("get_ore_status", JsValue::NULL).await;
                if let Ok(result) = result {
                    let result = result.as_string().unwrap();
                    let ore_status = TaskStatus::from(result);
                    state.app_config.update(|config| {
                        config.ore_status = Some(ore_status);
                    });
                }
            });
        },
        Duration::from_secs(10),
    );

    view! {
        <div class="h-screen bg-gray-800">
            <Suspense fallback=move || view! { <p>Loading</p> }>
                <div class="hidden">{resource.get()}</div>
                <div class="h-full">
                    <AppRouter/>
                    <AppFooter/>
                </div>
            </Suspense>
        </div>
    }
}
