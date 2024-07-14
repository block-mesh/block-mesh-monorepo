use crate::frontends::frontend_extension::extension_state::ExtensionState;
use crate::frontends::frontend_extension::utils::auth::login;
use crate::frontends::frontend_extension::utils::connectors::send_message_channel;
use block_mesh_common::chrome_storage::{ExtensionStatus, MessageKey, MessageType, MessageValue};
use block_mesh_common::interfaces::server_api::LoginForm;
use leptos::logging::log;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn ExtensionLogin() -> impl IntoView {
    let state = use_context::<ExtensionState>().unwrap();
    let (password, set_password) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());

    let submit_action_resource = create_local_resource(
        move || (),
        move |_| async move {
            if email.get_untracked().is_empty() || password.get_untracked().is_empty() {
                return;
            }
            let credentials = LoginForm {
                email: email.get_untracked(),
                password: password.get_untracked(),
            };

            let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
            match result {
                Ok(res) => {
                    if res.message.is_some() {
                        ExtensionState::set_error(res.message.unwrap(), state.error);
                        return;
                    }
                    if let Some(api_token) = res.api_token {
                        if api_token != state.api_token.get_untracked()
                            || state.api_token.get_untracked() == Uuid::default()
                        {
                            log!("Store new api token");
                            state.api_token.update(|v| *v = api_token);
                            state.email.update(|e| *e = credentials.email.clone());
                            send_message_channel(
                                MessageType::SET,
                                MessageKey::Email,
                                Option::from(MessageValue::String(state.email.get_untracked())),
                            )
                            .await;
                            send_message_channel(
                                MessageType::SET,
                                MessageKey::ApiToken,
                                Option::from(MessageValue::UUID(api_token)),
                            )
                            .await;
                        } else {
                            log!("Logged in");
                        }
                        ExtensionState::set_success("Successfully logged in", state.success);
                        state.status.update(|v| *v = ExtensionStatus::LoggedIn);
                    }
                }
                Err(err) => {
                    tracing::error!("Unable to login with {}: {err}", credentials.email);
                    ExtensionState::set_error(
                        "Failed to login, please check your credentials again",
                        state.error,
                    );
                }
            }
        },
    );

    view! {
        <div class="auth-card">
            <img
                class="background-image"
                src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/16475f13-7a36-4787-a076-580885250100/public"
                alt="background"
            />
            <div class="auth-card-frame"></div>
            <div class="auth-card-top"></div>
            <div class="auth-card-body">
                <img
                    class="h-16 w-16 m-auto"
                    src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                    alt="logo"
                />
                <h1>BlockMesh</h1>
                <form on:submit=|ev| ev.prevent_default()>
                    <div class="auth-card-input-container">
                        <input
                            type="text"
                            required=""
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

                        <label>Email</label>
                    </div>
                    <div class="auth-card-input-container">
                        <input
                            type="password"
                            required=""

                            name="password"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        submit_action_resource.refetch();
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

                        <label>Password</label>
                    </div>
                    <br/>
                    <button
                        class="auth-card-button"
                        on:click=move |_ev| {
                            submit_action_resource.refetch();
                        }
                    >

                        Login
                    </button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">Doesnt have an account yet?</small>
                <br/>
                <small
                    class="auth-card-link register-link"
                    on:click=move |_| { state.status.update(|v| *v = ExtensionStatus::Registering) }
                >
                    Register now
                </small>
            </div>
        </div>
    }
}
