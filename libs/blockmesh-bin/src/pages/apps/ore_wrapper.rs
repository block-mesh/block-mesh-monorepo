use crate::components::ore_icon::OreIcon;
use block_mesh_common::app_config::TaskStatus;
use leptos::*;
use std::str::FromStr;

use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::{invoke_tauri, SetAppConfigArgs, ToggleMinerArgs};

#[component]
pub fn OreWrapper() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let status = Signal::derive(move || state.app_config.get().ore_status);
    let (error, set_error) = create_signal(None::<String>);
    let ore_status = Signal::derive(move || {
        state
            .app_config
            .get()
            .ore_status
            .unwrap_or_default()
            .to_string()
    });
    let rpc_url = Signal::derive(move || state.app_config.get().ore_rpc);
    let threads = Signal::derive(move || state.app_config.get().ore_threads);
    let priority_fee = Signal::derive(move || state.app_config.get().ore_priority_fee);
    let keypair = Signal::derive(move || state.app_config.get().ore_keypair);

    let set_rpc_url = move |rpc_url: String| {
        state.app_config.update(|c| {
            c.ore_rpc = Some(rpc_url);
        })
    };

    let set_keypair = move |key: String| {
        state.app_config.update(|c| {
            c.ore_keypair = Some(key);
        })
    };

    let set_threads = move |threads: u16| {
        state.app_config.update(|c| {
            c.ore_threads = Some(threads);
        })
    };

    let set_priority_fee = move |priority_fee: u64| {
        state.app_config.update(|c| {
            c.ore_priority_fee = Some(priority_fee);
        })
    };

    let toggle_action = move || {
        spawn_local(async move {
            let state = expect_context::<LeptosTauriAppState>();
            let args = ToggleMinerArgs {
                task_status: match state.app_config.get().ore_status {
                    Some(TaskStatus::Running) => TaskStatus::Off,
                    Some(TaskStatus::Off) => TaskStatus::Running,
                    None => TaskStatus::Off,
                },
            };
            if let Ok(js_args) = serde_wasm_bindgen::to_value(&args) {
                let _ = invoke_tauri("toggle_miner", js_args).await;
                LeptosTauriAppState::get_ore_status(&state).await;
            }
        });
    };

    let submit_action = move || {
        spawn_local(async move {
            let conf = state.app_config.get_untracked();
            let args: SetAppConfigArgs = SetAppConfigArgs {
                config: conf.clone(),
            };
            if let Ok(js_args) = serde_wasm_bindgen::to_value(&args) {
                let result = invoke_tauri("set_app_config", js_args).await;
                match result {
                    Ok(_) => {
                        state.app_config.set(conf.clone());
                    }
                    Err(error) => {
                        set_error.update(|e| *e = Some(error.to_string()));
                    }
                }
            }
        });
    };

    let status_color = move || match state.app_config.get().ore_status {
        None => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-red-600",
        Some(ore_status) => match ore_status {
            TaskStatus::Running => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-green-600",
            TaskStatus::Off => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-red-600",
        },
    };

    view! {
        <div class="container mx-auto my-8">
            <div class="bg-gray-800 shadow-md rounded-lg overflow-hidden">
                <div class="px-4 py-5 sm:px-6 bg-gray-900">
                    <h2 class="text-xl leading-6 font-medium text-white">
                        <div class="flex items-center justify-center">
                            <OreIcon/>
                        </div>
                    </h2>
                </div>
                <div class="border-t border-white">
                    <dl>
                        <div class="bg-gray-700 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-500">Ore Miner</dt>
                            <dd class=status_color>{move || ore_status.get()}</dd>
                        </div>
                    </dl>
                </div>
            </div>
        </div>
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
                        <label class="block text-white text-sm font-bold mb-2">RPC URL</label>
                        <input
                            type="text"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="PRC URL"
                            name="rpc"
                            value=move || rpc_url.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_rpc_url(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_rpc_url(val);
                            }
                        />

                        <div class="mb-4">
                            <label class="block text-white text-sm font-bold mb-2">
                                Key Pair Path
                            </label>
                            <input
                                type="text"
                                // required
                                class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                                placeholder="Key Pair Path"
                                name="keypair"
                                value=move || keypair.get()
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    let val = event_target_value(&ev);
                                    set_keypair(val);
                                }

                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    set_keypair(val);
                                }
                            />

                        </div>
                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">Threads</label>
                        <input
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Threads"
                            name="threads"
                            min="1"
                            max="65535"
                            step="1"
                            value=move || threads.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_threads(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u16::from_str(&val).unwrap_or(0);
                                set_threads(val);
                            }
                        />

                    </div> <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2">Threads</label>
                        <input
                            type="number"
                            // required
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            placeholder="Priority Fee"
                            name="priority_fee"
                            min="1"
                            max="65535"
                            step="1"
                            value=move || priority_fee.get()
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                let val = u64::from_str(&val).unwrap_or(0);
                                set_priority_fee(val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                let val = u64::from_str(&val).unwrap_or(0);
                                set_priority_fee(val);
                            }
                        />

                    </div> <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| { submit_action() }
                        >
                            Save Config
                        </button>
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            on:click=move |_| { toggle_action() }
                        >
                            {move || {
                                status.get().map(|status| status.to_string()).unwrap_or_default()
                            }}

                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
