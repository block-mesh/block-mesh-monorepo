use crate::components::credentials::CredentialsForm;
use crate::utils::auth::register;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::interfaces::server_api::RegisterForm;
use leptos::{logging::log, *};
use leptos_dom::tracing;

#[component]
pub fn Register(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();
    let url = Signal::derive(move || state.blockmesh_url.get());

    let register_action = create_action(move |params: &Vec<String>| {
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

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <CredentialsForm
            url=url
            action_label="Register"
            action=register_action
            disabled=disabled
            register=true
        />
    }
}
