use crate::leptos_state::LeptosTauriAppState;
use block_mesh_common::app_config::AppConfig;
use leptos::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[component]
pub fn ProxyEndpointSettingsForm() -> impl IntoView {
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
            if let Ok(val) = Pubkey::from_str(&address) {
                c.proxy_master_node_owner = Some(val);
            }
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
            if let Ok(val) = Pubkey::from_str(&address) {
                c.program_id = Some(val);
            }
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

    let submit_action = move || {
        if state.app_config.get().mode.is_some() {
            let proxy_master_address = match Pubkey::from_str(&proxy_master_address.get()) {
                Ok(val) => Some(val),
                Err(_) => {
                    set_error.update(|e| *e = Some("Invalid Proxy Master Address".to_string()));
                    return;
                }
            };

            let program_address = Pubkey::from_str(&program_address.get());
            if program_address.is_err() {
                set_error.update(|e| *e = Some("Invalid Program Address".to_string()));
                return;
            }

            let config = AppConfig {
                keypair_path: keypair_path.get(),
                proxy_master_node_owner: proxy_master_address,
                program_id: Some(program_address.unwrap()),
                proxy_override: Some(proxy_override.get()),
                mode: state.app_config.get().mode,
                ..AppConfig::default()
            };
            state.app_config.set(config);
        }
    };

    view! {
        <form on:submit=|ev| ev.prevent_default()>
            <div class="flex justify-center items-center m-4">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    {move || {
                        error
                            .get()
                            .map(|err| {
                                view! { <p style="color:red;">{err}</p> }
                            })
                    }}
                    <div class="mb-4">
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
