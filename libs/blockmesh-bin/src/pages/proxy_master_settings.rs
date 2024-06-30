use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::{invoke_tauri, SetAppConfigArgs};
use block_mesh_common::app_config::AppConfig;
use leptos::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[component]
pub fn ProxyMasterSettingsForm() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let (error, set_error) = create_signal(None::<String>);

    let keypair_path = Signal::derive(move || state.app_config.get().keypair_path);
    let set_keypair_path =
        move |key: String| state.app_config.update(|c| c.keypair_path = Some(key));

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
            if let Ok(val) = Pubkey::from_str(&address) {
                c.program_id = Some(val);
            }
        });
    };

    let proxy_port = Signal::derive(move || state.app_config.get().proxy_port.unwrap_or(5000));
    let set_proxy_port = move |port: u16| {
        state.app_config.update(|c| c.proxy_port = Some(port));
    };

    let client_port = Signal::derive(move || state.app_config.get().client_port.unwrap_or(4000));
    let set_client_port = move |port: u16| {
        state.app_config.update(|c| c.client_port = Some(port));
    };

    let submit_action = move || {
        let program_address = Pubkey::from_str(&program_address.get());
        if program_address.is_err() {
            set_error.update(|e| *e = Some("Invalid Program Address".to_string()));
            return;
        }
        let config = AppConfig {
            keypair_path: keypair_path.get(),
            program_id: Some(program_address.unwrap()),
            proxy_port: Some(proxy_port.get()),
            client_port: Some(client_port.get()),
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

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">Client Port</label>
                        <input
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Client Port"
                            name="client_port"
                            min="1"
                            max="65535"
                            step="1"
                            value=move || client_port.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_client_port(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_client_port(val);
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
