use crate::state::LeptosTauriAppState;
use block_mesh_common::cli::Commands;
use block_mesh_common::constants::BLOCK_MESH_PROGRAM_ID;
use leptos::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[component]
pub fn ProxyMasterSettingsForm() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let command = move || state.cli_args.get_untracked().command;
    let (error, set_error) = create_signal(None::<String>);
    let (keypair_path, set_keypair_path) = create_signal(match command() {
        Some(Commands::ProxyMaster(options)) => options.keypair_path.to_string(),
        _ => "".to_string(),
    });
    let (program_address, set_program_address) = create_signal(match command() {
        Some(Commands::ProxyMaster(options)) => options.program_id.to_string(),
        _ => BLOCK_MESH_PROGRAM_ID.to_string(),
    });

    let (proxy_port, set_proxy_port) = create_signal(match command() {
        Some(Commands::ProxyMaster(options)) => options.proxy_port,
        _ => 5000,
    });
    let (client_port, set_client_port) = create_signal(match command() {
        Some(Commands::ProxyMaster(options)) => options.client_port,
        _ => 4000,
    });

    let submit_action = move || {
        if let Some(cmd) = state.cli_args.get().command {
            let program_address = Pubkey::from_str(&program_address.get());
            if program_address.is_err() {
                set_error.update(|e| *e = Some("Invalid Program Address".to_string()));
                return;
            }

            if let Commands::ProxyMaster(mut options) = cmd {
                options.keypair_path = keypair_path.get();
                options.program_id = program_address.unwrap();
                options.proxy_port = proxy_port.get();
                options.client_port = client_port.get();
                let mut args = state.cli_args.get();
                args.command = Some(Commands::ProxyMaster(options));
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
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Port"
                            name="proxy_port"
                            min="1"
                            max="65535"
                            step="1"
                            prop:disabled=move || command().is_none()
                            value=move || proxy_port.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_proxy_port.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_proxy_port.update(|v| *v = val);
                            }
                        />

                    </div> <div class="mb-4">
                        <input
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Proxy Port"
                            name="proxy_port"
                            min="1"
                            max="65535"
                            step="1"
                            prop:disabled=move || command().is_none()
                            value=move || client_port.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_client_port.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_client_port.update(|v| *v = val);
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
