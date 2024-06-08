use crate::app::{invoke_tauri, SetAppConfigArgs};
use crate::leptos_state::LeptosTauriAppState;
use block_mesh_common::app_config::AppConfig;
use leptos::*;

#[component]
pub fn ToggleButton() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let enabled = create_rw_signal(state.app_config.get().enable_node.unwrap_or(false));
    let button_move_class = Signal::derive(move || {
        if enabled.get() {
            "translate-x-5"
        } else {
            "translate-x-0"
        }
    });

    let submit_action = move || {
        let config = AppConfig {
            enable_node: Some(enabled.get()),
            ..state.app_config.get().clone()
        };

        spawn_local(async move {
            let args: SetAppConfigArgs = SetAppConfigArgs {
                config: config.clone(),
            };
            if let Ok(js_args) = serde_wasm_bindgen::to_value(&args) {
                let result = invoke_tauri("set_app_config", js_args).await;
                match result {
                    Ok(_) => {
                        state.app_config.set(config.clone());
                    }
                    Err(_error) => {
                        // set_error.update(|e| *e = Some(error.to_string()));
                    }
                }
            }
        });
    };

    view! {
        <div class="flex items-center justify-center">
            <div class="flex items-center space-x-4">
                <label class="text-white">Enable Node</label>
                <button
                    type="button"
                    class=move || {
                        format!(
                            "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2 {}",
                            if enabled.get() { "bg-indigo-600" } else { "bg-gray-200" },
                        )
                    }

                    role="switch"
                    aria-checked="false"
                    on:click=move |_| {
                        if enabled.get() {
                            enabled.set(false);
                        } else {
                            enabled.set(true);
                        }
                        submit_action()
                    }
                >

                    <span class="sr-only">Use setting</span>
                    <span class=move || {
                        format!(
                            "pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out {}",
                            button_move_class.get(),
                        )
                    }>
                        <span
                            class=move || {
                                format!(
                                    "absolute inset-0 flex h-full w-full items-center justify-center transition-opacity duration-200  {}",
                                    if enabled.get() {
                                        "opacity-0 duration-100 ease-out"
                                    } else {
                                        "opacity-100 duration-200 ease-in"
                                    },
                                )
                            }

                            aria-hidden="true"
                        >
                            <svg class="h-3 w-3 text-gray-400" fill="none" viewBox="0 0 12 12">
                                <path
                                    d="M4 8l2-2m0 0l2-2M6 6L4 4m2 2l2 2"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                ></path>
                            </svg>
                        </span>
                        <span
                            class=move || {
                                format!(
                                    "absolute inset-0 flex h-full w-full items-center justify-center transition-opacity {}",
                                    if enabled.get() {
                                        "opacity-100 duration-200 ease-in"
                                    } else {
                                        "opacity-0 duration-100 ease-out"
                                    },
                                )
                            }

                            aria-hidden="true"
                        >
                            <svg
                                class="h-3 w-3 text-indigo-600"
                                fill="currentColor"
                                viewBox="0 0 12 12"
                            >
                                <path d="M3.707 5.293a1 1 0 00-1.414 1.414l1.414-1.414zM5 8l-.707.707a1 1 0 001.414 0L5 8zm4.707-3.293a1 1 0 00-1.414-1.414l1.414 1.414zm-7.414 2l2 2 1.414-1.414-2-2-1.414 1.414zm3.414 2l4-4-1.414-1.414-4 4 1.414 1.414z"></path>
                            </svg>
                        </span>
                    </span>
                </button>
            </div>
        </div>
    }
}
