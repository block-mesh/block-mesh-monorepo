use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::utils::auth::login;
use crate::frontends::utils::connectors::send_message_channel;
use block_mesh_common::chrome_storage::{AuthStatus, MessageKey, MessageType, MessageValue};
use block_mesh_common::interfaces::server_api::LoginForm;
use leptos::leptos_dom::log;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn TauriLogin() -> impl IntoView {
    let state = expect_context::<AuthContext>();
    let notifications = expect_context::<NotificationContext>();

    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (wait, set_wait) = create_signal(false);

    let submit_action_resource = create_local_resource(
        move || (),
        move |_| async move {
            log!(
                "starting wait {} email {} password {} url {}",
                wait.get_untracked(),
                email.get_untracked(),
                password.get_untracked(),
                state.blockmesh_url.get_untracked()
            );
            if wait.get_untracked()
                || email.get_untracked().is_empty()
                || password.get_untracked().is_empty()
            {
                return;
            }
            set_wait.set(true);
            let credentials = LoginForm {
                email: email.get_untracked(),
                password: password.get_untracked(),
            };

            let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
            log!("result {:#?}", result);
            match result {
                Ok(res) => {
                    if res.message.is_some() {
                        notifications.set_error(res.message.unwrap());
                        set_wait.set(false);
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
                                Some(MessageValue::String(state.email.get_untracked())),
                            )
                            .await;
                            send_message_channel(
                                MessageType::SET,
                                MessageKey::ApiToken,
                                Some(MessageValue::UUID(api_token)),
                            )
                            .await;
                        }

                        notifications.set_success("Successfully logged in");

                        match AuthContext::load_account_data().await {
                            Ok(account_data) => {
                                state
                                    .wallet_address
                                    .update(|v| *v = account_data.wallet_address);
                                send_message_channel(
                                    MessageType::SET,
                                    MessageKey::WalletAddress,
                                    state
                                        .wallet_address
                                        .get_untracked()
                                        .map(MessageValue::String),
                                )
                                .await;
                            }
                            Err(e) => {
                                notifications
                                    .set_error(format!("Failed to load account data : {e:?}"));
                            }
                        }

                        state.status.update(|v| *v = AuthStatus::LoggedIn);
                    }
                }
                Err(e) => {
                    notifications.set_error(format!(
                        "Failed to login, please check your credentials again : {:?}",
                        e
                    ));
                }
            }
            set_wait.set(false);
        },
    );

    view! {
        <div>
            <div class="bg-dark-blue flex justify-center items-center h-screen">
                <div class="bg-dark-blue border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                    <h2 class="font-bebas-neue text-off-white text-2xl font-semibold text-center mb-6">
                        Login
                    </h2>
                    <div class="flex justify-around mb-4">
                        <button
                            class="cursor-pointer font-bebas-neue px-4 py-2 rounded font-bold text-sm text-cyan hover:text-orange"
                            on:click=move |_| {
                                state.status.update(|v| *v = AuthStatus::Registering)
                            }
                        >

                            Register
                        </button>
                    </div>
                    <div class="mb-4">
                        <label
                            class="font-bebas-neue block text-off-white text-sm font-bold mb-2"
                            for="email"
                        >
                            Email
                        </label>
                        <input
                            autocapitalize="off"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-black leading-tight focus:outline-none focus:shadow-outline"
                            type="text"
                            id="email"
                            placeholder="Email"
                            name="email"
                            on:keyup=move |ev: ev::KeyboardEvent| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val.to_ascii_lowercase());
                            }

                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_email.update(|v| *v = val.to_ascii_lowercase());
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
                            autocapitalize="off"
                            class="shadow appearance-none border rounded w-full py-2 px-3 text-black mb-3 leading-tight focus:outline-none focus:shadow-outline"
                            type="password"
                            id="password"
                            name="password"
                            placeholder="******************"
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
                    <div class="flex items-center justify-between">
                        <button
                            class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_| { submit_action_resource.refetch() }
                        >
                            Submit
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
