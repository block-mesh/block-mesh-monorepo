use crate::frontends::frontend_extension::extension_state::ExtensionState;
use crate::frontends::frontend_extension::utils::auth::register;
use crate::frontends::frontend_extension::utils::connectors::send_message_channel;
use block_mesh_common::chrome_storage::{ExtensionStatus, MessageKey, MessageType, MessageValue};
use block_mesh_common::interfaces::server_api::RegisterForm;
use leptos::*;
use leptos_dom::tracing;
use leptos_router::A;

#[component]
pub fn ExtensionRegister() -> impl IntoView {
    let state = use_context::<ExtensionState>().unwrap();
    let (password, set_password) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (invite_code, set_invite_code) = create_signal(String::new());

    let submit_action_resource = create_local_resource(
        move || (),
        move |_| async move {
            if email.get_untracked().is_empty() || password.get_untracked().is_empty() {
                return;
            }
            let credentials = RegisterForm {
                email: email.get_untracked(),
                password: password.get_untracked(),
                password_confirm: password.get_untracked(),
                invite_code: if invite_code.get_untracked().is_empty() {
                    None
                } else {
                    Some(invite_code.get_untracked())
                },
            };
            let result = register(&state.blockmesh_url.get_untracked(), &credentials).await;
            match result {
                Ok(_) => {
                    state.api_token.update(|t| *t = uuid::Uuid::default());
                    send_message_channel(
                        MessageType::SET,
                        MessageKey::ApiToken,
                        Option::from(MessageValue::UUID(uuid::Uuid::default())),
                    )
                    .await;
                    state
                        .status
                        .update(|v| *v = ExtensionStatus::WaitingEmailVerification);
                    ExtensionState::set_success("Please confirm email and login", state.success);
                }
                Err(err) => {
                    tracing::error!(
                        "Unable to register new account for {}: {err}",
                        credentials.email
                    );
                    ExtensionState::set_error(err.to_string(), state.error);
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
                <div class="flex justify-center">

                    <img
                        class="h-16 w-16 m-auto"
                        src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                        alt="logo"
                    />
                </div>
                <h1>BlockMesh</h1>
                <form on:submit=|ev| ev.prevent_default()>
                    <div class="auth-card-input-container">
                        <input
                            type="text"
                            required=""
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
                    <div class="auth-card-input-container">
                        <input
                            type="text"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_invite_code.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_invite_code.update(|v| *v = val);
                            }
                        />

                        <label>Refer Code</label>
                    </div>
                    <button
                        class="auth-card-button"
                        on:click=move |_ev| submit_action_resource.refetch()
                    >
                        Register
                    </button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">You already have an account?</small>
                <br/>
                <small
                    class="auth-card-link register-link"
                    on:click=move |_| { state.status.update(|v| *v = ExtensionStatus::LoggedOut) }
                >
                    <A href="/ext/login">Login now</A>
                </small>
            </div>
        </div>
    }
}
