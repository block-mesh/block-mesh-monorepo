use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::{invoke_tauri, RegisterArgs};
use block_mesh_common::interfaces::server_api::RegisterForm;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Register() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (password_confirm, set_password_confirm) = create_signal(String::new());
    let (invite_code, set_invite_code) = create_signal(String::new());

    let submit_action = move || {
        spawn_local(async move {
            let e = email.get_untracked();
            let p = password.get_untracked();
            let pc = password_confirm.get_untracked();
            let i = invite_code.get_untracked();
            let args: RegisterArgs = RegisterArgs {
                register_form: RegisterForm {
                    email: e.clone(),
                    password: p,
                    password_confirm: pc.clone(),
                    invite_code: if i.is_empty() { None } else { Some(i) },
                },
            };
            if let Ok(js_args) = serde_wasm_bindgen::to_value(&args) {
                if invoke_tauri("register", js_args).await.is_ok() {
                    LeptosTauriAppState::set_success(
                        "Please verify email and login".to_string(),
                        state.success,
                    );
                }
            }
        });
    };

    view! {
        <div>
            <div class="bg-gray-700 flex justify-center items-center h-screen">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="text-white text-2xl font-semibold text-center mb-6">Register</h2>
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="email">
                            Email
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                            type="text"
                            id="email"
                            placeholder="Email"
                            name="email"
                            required
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val);
                            }
                        />

                    </div>
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="password">
                            Password
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password"
                            name="password"
                            placeholder="******************"
                            required

                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        submit_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_password.update(|p| *p = val);
                                    }
                                }
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password.update(|p| *p = val);
                            }
                        />

                    </div>
                    <div class="mb-4">
                        <label
                            class="block text-white text-sm font-bold mb-2"
                            for="password_confirm"
                        >
                            Confirm
                            Password
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password_confirm"
                            name="password_confirm"
                            placeholder="******************"
                            required

                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        submit_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_password_confirm.update(|p| *p = val);
                                    }
                                }
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_password_confirm.update(|p| *p = val);
                            }
                        />

                    </div>
                    <div class="mb-4">
                        <label class="block text-white text-sm font-bold mb-2" for="invite_code">
                            Invite Code
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="text"
                            id="invite_code"
                            name="invite_code"
                            placeholder="Invite Code"

                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        submit_action();
                                    }
                                    _ => {
                                        let val = event_target_value(&ev);
                                        set_invite_code.update(|p| *p = val);
                                    }
                                }
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_invite_code.update(|p| *p = val);
                            }
                        />

                    </div>
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_| { submit_action() }
                        >
                            Submit
                        </button>
                        <A
                            class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800"
                            href="/login"
                        >
                            Login
                        </A>
                    </div>
                </div>
            </div>
        </div>
    }
}
