use crate::leptos_state::LeptosTauriAppState;
use crate::tauri_connector::connector::{invoke_tauri, LoginArgs};
use block_mesh_common::app_config::AppConfig;
use block_mesh_common::interfaces::server_api::{GetTokenResponse, LoginForm};
use leptos::*;
use leptos_router::A;

#[component]
pub fn Login() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    let submit_action = create_action(move |_| async move {
        let e = email.get_untracked();
        let p = password.get_untracked();
        let args: LoginArgs = LoginArgs {
            login_form: LoginForm {
                email: e.clone(),
                password: p,
            },
        };
        if let Ok(js_args) = serde_wasm_bindgen::to_value(&args) {
            if let Ok(result) = invoke_tauri("login", js_args).await {
                if let Ok(value) = serde_wasm_bindgen::from_value::<GetTokenResponse>(result) {
                    let conf = state.app_config.get_untracked();
                    state.app_config.update(|v| {
                        *v = AppConfig {
                            api_token: Some(value.api_token.unwrap_or_default().to_string()),
                            email: Some(e),
                            ..conf
                        }
                    });
                    state.logged_in.update(|v| *v = true);
                    LeptosTauriAppState::set_success("Successful login".to_string(), state.success);
                }
            }
        }
    });

    let on_submit_action = move || {
        submit_action.dispatch(());
    };

    view! {
        <div>
            <div class="bg-gray-700 flex justify-center items-center h-screen">
                <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="text-white text-2xl font-semibold text-center mb-6">Login</h2>
                    <div class="flex justify-around mb-4">
                        <A
                            class="px-4 py-2 rounded font-bold text-sm text-blue-500 hover:text-blue-800"
                            href="/register"
                        >
                            Register
                        </A>
                    </div>
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
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        on_submit_action();
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
                    <div class="flex items-center justify-between">
                        <button
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_| { on_submit_action() }
                        >
                            Submit
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
