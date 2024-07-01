use crate::utils::auth::register;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::interfaces::server_api::RegisterForm;
use leptos::{logging::log, *};
use leptos_dom::tracing;
use leptos_router::A;

#[component]
pub fn Register(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();
    let _url = Signal::derive(move || state.blockmesh_url.get());

    let _register_action = create_action(move |params: &Vec<String>| {
        let email = params[0].to_string();
        let password = params[1].to_string();
        let password_confirm = params[2].to_string();
        let invite_code = params[3].to_string();
        let credentials = RegisterForm {
            email,
            password,
            password_confirm,
            invite_code: if invite_code.is_empty() {
                None
            } else {
                Some(invite_code)
            },
        };
        log!("Try to register new account for {}", credentials.email);
        async move {
            if credentials.password != credentials.password_confirm {
                tracing::warn!("Passwords do not match");
                AppState::set_error("Passwords do not match".to_string(), state.error);
                return;
            }
            set_wait_for_response.update(|w| *w = true);
            let result = register(&state.blockmesh_url.get_untracked(), &credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(_) => {
                    state.api_token.update(|t| *t = uuid::Uuid::default());
                    AppState::store_api_token(uuid::Uuid::default()).await;
                    state
                        .status
                        .update(|v| *v = AppStatus::WaitingEmailVerification);
                    on_success.call(());
                }
                Err(err) => {
                    tracing::error!(
                        "Unable to register new account for {}: {err}",
                        credentials.email
                    );
                    AppState::set_error(err.to_string(), state.error);
                }
            }
        }
    });

    let _disabled = Signal::derive(move || wait_for_response.get());

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
                        <input type="text" required=""/>
                        <label>Email</label>
                    </div>
                    <div class="auth-card-input-container">
                        <input type="password" required=""/>
                        <label>Password</label>
                    </div>
                    <div class="auth-card-input-container">
                        <input type="text" required=""/>
                        <label>Refer Code</label>
                    </div>
                    <button class="auth-card-button">Register</button>
                </form>
            </div>
            <div class="auth-card-bottom">
                <small class="auth-card-sub-text">You already have an account?</small>
                <br/>
                <small class="auth-card-link register-link">
                    <A href="/login">Login now</A>
                </small>
            </div>
        </div>
    }
}
