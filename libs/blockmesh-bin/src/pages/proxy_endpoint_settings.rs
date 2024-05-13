use crate::state::LeptosTauriAppState;
use block_mesh_common::cli::Commands;
use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use leptos::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[component]
pub fn ProxyEndpointSettingsForm() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let command = move || state.cli_args.get_untracked().command;
    let (error, set_error) = create_signal(None::<String>);

    let (keypair_path, set_keypair_path) = create_signal(match command() {
        Some(Commands::ProxyEndpoint(options)) => options.keypair_path.to_string(),
        _ => "".to_string(),
    });

    let (proxy_master_address, set_proxy_master_address) = create_signal(match command() {
        Some(Commands::ProxyEndpoint(options)) => {
            options.proxy_master_node_owner.map(|v| v.to_string())
        }
        _ => None,
    });
    let (program_address, set_program_address) = create_signal(match command() {
        Some(Commands::ProxyEndpoint(options)) => options.program_id.to_string(),
        _ => BLOCK_MESH_PROGRAM_ID.to_string(),
    });

    let (proxy_override, set_proxy_override) = create_signal(match command() {
        Some(Commands::ProxyEndpoint(options)) => options.proxy_override,
        _ => None,
    });

    let submit_action = move || {
        if let Some(cmd) = state.cli_args.get().command {
            let proxy_master_address = match proxy_master_address.get() {
                None => None,
                Some(val) => {
                    let proxy_master_address = Pubkey::from_str(&val);
                    match proxy_master_address {
                        Ok(val) => Some(val),
                        Err(_) => {
                            set_error
                                .update(|e| *e = Some("Invalid Proxy Master Address".to_string()));
                            return;
                        }
                    }
                }
            };

            let program_address = Pubkey::from_str(&program_address.get());
            if program_address.is_err() {
                set_error.update(|e| *e = Some("Invalid Program Address".to_string()));
                return;
            }

            if let Commands::ProxyEndpoint(mut options) = cmd {
                options.keypair_path = keypair_path.get();
                options.proxy_master_node_owner = proxy_master_address;
                options.program_id = program_address.unwrap();
                options.proxy_override = proxy_override.get();
                let mut args = state.cli_args.get();
                args.command = Some(Commands::ProxyEndpoint(options));
                state.cli_args.set(args);
            }
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
                            prop:disabled=move || command().is_none()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_keypair_path.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_keypair_path.update(|v| *v = val);
                            }
                        />

                    </div> <div class="mb-4">
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Master Address"
                            name="proxy_master_address"
                            prop:disabled=move || command().is_none()
                            value=move || proxy_master_address.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_proxy_master_address.update(|v| *v = Some(val));
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_proxy_master_address.update(|v| *v = Some(val));
                            }
                        />

                    </div> <div class="mb-4">
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Program Address"
                            name="program_address"
                            prop:disabled=move || command().is_none()
                            value=move || program_address.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_program_address.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_program_address.update(|v| *v = val);
                            }
                        />

                    </div> <div class="mb-4">
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Override"
                            name="proxy_override"
                            prop:disabled=move || command().is_none()
                            value=move || proxy_override.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_proxy_override.update(|v| *v = Some(val));
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_proxy_override.update(|v| *v = Some(val));
                            }
                        />

                    </div> <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            prop:disabled=move || command().is_none()
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
