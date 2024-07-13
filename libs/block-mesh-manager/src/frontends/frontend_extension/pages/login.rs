use crate::frontends::frontend_extension::components::notification::Notifications;
use crate::frontends::frontend_extension::extension_state::{AppStatus, ExtensionState};
use crate::frontends::frontend_extension::utils::auth::login;
use crate::frontends::frontend_extension::utils::connectors::send_message_channel;
use block_mesh_common::chrome_storage::{StorageMessageType, StorageValue, StorageValues};
use block_mesh_common::interfaces::server_api::LoginForm;
use leptos::logging::log;
use leptos::*;
use leptos_router::{use_navigate, A};
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
            log!("blockmesh_url {}", state.blockmesh_url.get_untracked());
            let credentials = LoginForm {
                email: email.get_untracked(),
                password: password.get_untracked(),
            };

            let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
            log!("result {:#?}", result);
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
                                StorageMessageType::SET,
                                StorageValues::Email,
                                Option::from(StorageValue::String(state.email.get_untracked())),
                            )
                            .await;
                            send_message_channel(
                                StorageMessageType::SET,
                                StorageValues::ApiToken,
                                Option::from(StorageValue::UUID(api_token)),
                            )
                            .await;
                        } else {
                            log!("Logged in");
                        }
                        log!("navigate");
                        state.status.update(|v| *v = AppStatus::LoggedIn);
                        let navigate = use_navigate();
                        navigate("/ext/logged_in", Default::default());
                    }
                }
                Err(_err) => {
                    // tracing::error!("Unable to login with {}: {err}", credentials.email);
                    // AppState::set_error(
                    //     "Failed to login, please check your credentials again",
                    //     state.error,
                    // );
                }
            }
        },
    );

    // let submit_action = create_action(move |_| async move {
    //     let state = use_context::<ExtensionState>().unwrap();
    //     let credentials = LoginForm {
    //         email: email.get_untracked(),
    //         password: password.get_untracked(),
    //     };
    //
    //     let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
    //     match result {
    //         Ok(res) => {
    //             if res.message.is_some() {
    //                 ExtensionState::set_error(res.message.unwrap(), state.error);
    //                 return;
    //             }
    //             if let Some(api_token) = res.api_token {
    //                 if api_token != state.api_token.get_untracked()
    //                     || state.api_token.get_untracked() == Uuid::default()
    //                 {
    //                     tracing::info!("Store new api token");
    //                     state.api_token.update(|v| *v = api_token);
    //                     state.email.update(|e| *e = credentials.email.clone());
    //                     send_message_channel(
    //                         StorageMessageType::SET,
    //                         StorageValues::Email,
    //                         Option::from(StorageValue::String(state.email.get_untracked())),
    //                     )
    //                     .await;
    //                     send_message_channel(
    //                         StorageMessageType::SET,
    //                         StorageValues::ApiToken,
    //                         Option::from(StorageValue::UUID(api_token)),
    //                     )
    //                     .await;
    //                 } else {
    //                     tracing::info!("Logged in");
    //                 }
    //                 state.status.update(|v| *v = AppStatus::LoggedIn);
    //                 let navigate = use_navigate();
    //                 navigate("/ext/logged_in", Default::default());
    //             }
    //         }
    //         Err(_err) => {
    //             // tracing::error!("Unable to login with {}: {err}", credentials.email);
    //             // AppState::set_error(
    //             //     "Failed to login, please check your credentials again",
    //             //     state.error,
    //             // );
    //         }
    //     }
    // });

    view! {
        <Notifications/>
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
                    class="logo"
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
                                    "Enter" => { submit_action_resource.refetch(); },
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
                    <button class="auth-card-button" on:click=move |_ev| submit_action_resource.refetch() >
                        Login
                    </button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">Doesnt have an account yet?</small>
                <br/>
                <small class="auth-card-link register-link">
                    <A href="/ext/register">Register now</A>
                </small>
            </div>
        </div>
    }
}
