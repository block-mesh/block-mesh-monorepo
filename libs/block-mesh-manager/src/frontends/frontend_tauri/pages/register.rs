use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::frontend_extension::utils::auth::register;
use crate::frontends::frontend_extension::utils::connectors::send_message_channel;
use block_mesh_common::chrome_storage::{AuthStatus, MessageKey, MessageType, MessageValue};
use block_mesh_common::interfaces::server_api::RegisterForm;
use leptos::*;
use leptos_dom::tracing;

#[component]
pub fn TauriRegister() -> impl IntoView {
    let state = expect_context::<AuthContext>();
    let notifications = expect_context::<NotificationContext>();
    let (password, set_password) = create_signal(String::new());
    let (password_confirm, set_password_confirm) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (invite_code, set_invite_code) = create_signal(String::new());
    let (wait, set_wait) = create_signal(false);

    let submit_action_resource = create_local_resource(
        move || (),
        move |_| async move {
            if wait.get_untracked()
                || email.get_untracked().is_empty()
                || password.get_untracked().is_empty()
                || password_confirm.get_untracked().is_empty()
                || invite_code.get_untracked().is_empty()
            {
                return;
            }
            if password.get_untracked() != password_confirm.get_untracked() {
                notifications.set_error("Password doesnt match confirmation");
                return;
            }
            set_wait.set(true);
            let credentials = RegisterForm {
                email: email.get_untracked(),
                password: password.get_untracked(),
                password_confirm: password.get_untracked(),
                invite_code: invite_code.get_untracked(),
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
                        .update(|v| *v = AuthStatus::WaitingEmailVerification);
                    notifications.set_success("Please confirm email and login");
                }
                Err(err) => {
                    tracing::error!(
                        "Unable to register new account for {}: {err}",
                        credentials.email
                    );
                    notifications.set_error(err.to_string());
                }
            }
            set_wait.set(false);
        },
    );

    view! {
        <div class="bg-dark-blue h-screen">
            <div class="bg-dark-blue flex justify-center items-center h-screen">
                <div class="bg-dark-blue border-off-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="font-bebas-neue text-off-white text-2xl font-semibold text-center mb-6">
                        Register
                    </h2>
                    <div class="mb-4">
                        <label
                            class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                            for="email"
                        >
                            Email
                        </label>
                        <input
                            class="text-black shadow appearance-none border rounded w-full py-2 px-3 leading-tight focus:outline-none focus:shadow-outline"
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
                        <label
                            class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                            for="password"
                        >
                            Password
                        </label>
                        <input
                            class="text-black shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password"
                            name="password"
                            placeholder="******************"
                            required
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

                    </div>
                    <div class="mb-4">
                        <label
                            class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                            for="password_confirm"
                        >
                            Confirm
                            Password
                        </label>
                        <input
                            class="text-black shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password_confirm"
                            name="password_confirm"
                            placeholder="******************"
                            required

                            on:keyup=move |ev: ev::KeyboardEvent| {
                                match &*ev.key() {
                                    "Enter" => {
                                        submit_action_resource.refetch();
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
                        <label
                            class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                            for="invite_code"
                        >
                            Invite Code
                        </label>
                        <input
                            class="shadow appearance-none border rounded w-full py-2 px-3 mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="text"
                            id="invite_code"
                            name="invite_code"
                            placeholder="Invite Code"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_invite_code.update(|v| *v = val);
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_invite_code.update(|v| *v = val);
                            }
                        />

                    </div>
                    <div class="flex items-center justify-between">
                        <button
                            class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_ev| {
                                submit_action_resource.refetch();
                            }
                        >

                            Submit
                        </button>
                        <button
                            class="cursor-pointer font-open-sans mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:text-cyan"
                            on:click=move |_| {
                                state.status.update(|v| *v = AuthStatus::LoggedOut)
                            }
                        >

                            Login
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
