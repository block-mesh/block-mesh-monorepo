use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::{invoke_tauri, SetAppConfigArgs};
use block_mesh_common::app_config::AppConfig;
use leptos::*;
use std::str::FromStr;

#[component]
pub fn ClientNodeSettingsForm() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let (error, set_error) = create_signal(None::<String>);
    let keypair_path = Signal::derive(move || state.app_config.get().keypair_path);
    let set_keypair_path =
        move |key: String| state.app_config.update(|c| c.keypair_path = Some(key));
    let proxy_master_address = Signal::derive(move || {
        state
            .app_config
            .get()
            .proxy_master_node_owner
            .unwrap_or_default()
            .to_string()
    });
    let set_proxy_master_address = move |address: String| {
        state.app_config.update(|c| {
            c.proxy_master_node_owner = Some(address);
        });
    };

    let program_address = Signal::derive(move || {
        state
            .app_config
            .get()
            .program_id
            .unwrap_or_default()
            .to_string()
    });
    let set_program_address = move |address: String| {
        state.app_config.update(|c| {
            c.program_id = Some(address);
        });
    };
    let proxy_override = Signal::derive(move || {
        state
            .app_config
            .get()
            .proxy_override
            .unwrap_or_default()
            .to_string()
    });
    let set_proxy_override = move |override_: String| {
        state
            .app_config
            .update(|c| c.proxy_override = Some(override_));
    };
    let proxy_port = Signal::derive(move || state.app_config.get().proxy_port.unwrap_or(8100));
    let set_proxy_port = move |port: u16| {
        state.app_config.update(|c| c.proxy_port = Some(port));
    };

    let submit_action = move || {
        if state.app_config.get().mode.is_some() {
            let proxy_master_address = if proxy_master_address.get().is_empty() {
                return;
            } else {
                proxy_master_address.get()
            };

            let program_address = program_address.get();
            if program_address.is_empty() {
                set_error.update(|e| *e = Some("Invalid Program Address".to_string()));
                return;
            }

            let config = AppConfig {
                keypair_path: keypair_path.get(),
                proxy_master_node_owner: Option::from(proxy_master_address),
                program_id: Some(program_address),
                proxy_override: Some(proxy_override.get()),
                proxy_port: Some(proxy_port.get()),
                mode: state.app_config.get().mode,
                ..AppConfig::default()
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
                        Err(error) => {
                            set_error.update(|e| *e = Some(error.to_string()));
                        }
                    }
                }
            });
        }
    };

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="flex justify-center items-center m-4">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-full">
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">Key Pair Path</label>
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Key Pair Path"
                            name="keypair_path"
                            value=move || keypair_path.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_keypair_path(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_keypair_path(val);
                            }
                        />

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">
                            Proxy Master Address
                        </label>
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Master Address"
                            name="proxy_master_address"

                            value=move || proxy_master_address.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_proxy_master_address(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_proxy_master_address(val);
                            }
                        />

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">
                            Program Address
                        </label>
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Program Address"
                            name="program_address"
                            value=move || program_address.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_program_address(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_program_address(val);
                            }
                        />

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">
                            Proxy Override
                        </label>
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Override"
                            name="proxy_override"
                            value=move || proxy_override.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_proxy_override(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_proxy_override(val);
                            }
                        />

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">Proxy Port</label>
                        <input
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Port"
                            name="proxy_port"
                            min="1"
                            max="65535"
                            step="1"
                            value=move || proxy_port.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_proxy_port(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_proxy_port(val);
                            }
                        />

                    </div> <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| { submit_action() }
                        >
                            Submit
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
