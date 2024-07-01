use crate::utils::auth::login;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::interfaces::server_api::LoginForm;
use leptos::*;
use leptos_dom::tracing;
use leptos_router::A;
use uuid::Uuid;

#[component]
pub fn Login(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();
    let _url = Signal::derive(move || state.blockmesh_url.get());
    let login_action = create_action(move |params: &Vec<String>| {
        let email = params[0].to_string();
        tracing::info!(
            "Try to login with {} - {}",
            email,
            state.blockmesh_url.get_untracked()
        );
        let password = params[1].to_string();
        let credentials = LoginForm { email, password };
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    state.email.update(|e| *e = credentials.email.clone());
                    AppState::store_email(credentials.email).await;
                    if res.message.is_some() {
                        AppState::set_error(res.message.unwrap(), state.error);
                        return;
                    }
                    if let Some(api_token) = res.api_token {
                        if api_token != state.api_token.get_untracked()
                            || state.api_token.get_untracked() == Uuid::default()
                        {
                            tracing::info!("Store new api token");
                            AppState::store_api_token(api_token).await;
                        } else {
                            tracing::info!("Logged in");
                        }
                        state.status.update(|v| *v = AppStatus::LoggedIn);
                        on_success.call(());
                    }
                }
                Err(err) => {
                    tracing::error!("Unable to login with {}: {err}", credentials.email);
                    AppState::set_error(
                        "Failed to login, please check your credentials again",
                        state.error,
                    );
                }
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <div class="auth-card">
            <img class="background-image" src="/assets/background.png" alt="background"/>
            <div class="auth-card-frame"></div>
            <div class="auth-card-top"></div>
            <div class="auth-card-body">
                <img class="logo" src="/assets/block-mesh-logo.png" alt="logo"/>
                <h1>BlockMesh</h1>
                <form>
                    <div class="auth-card-input-container">
                        <input
                            type="text"
                            required=""

                            prop:disabled=move || disabled.get()
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
                                        login_action.dispatch(vec![email.get(), password.get()]);
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
                        on:click=move |_| login_action.dispatch(vec![email.get(), password.get()])
                    >
                        Login
                    </button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">Doesnt have an account yet?</small>
                <br/>
                <small class="auth-card-link register-link">
                    <A href="/register">Register now</A>
                </small>
        <small class="auth-card-link register-link">
                    <A href="/">home</A>
                </small>
            </div>
        </div>
    }
}
