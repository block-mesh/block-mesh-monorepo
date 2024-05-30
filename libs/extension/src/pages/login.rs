use crate::components::credentials::CredentialsForm;
use crate::utils::auth::login;
use crate::utils::ext_state::{AppState, AppStatus};
use block_mesh_common::interfaces::server_api::LoginForm;
use leptos::*;
use uuid::Uuid;

#[component]
pub fn Login(#[prop(into)] on_success: Callback<()>) -> impl IntoView {
    let (login_error, set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();
    let url = Signal::derive(move || state.blockmesh_url.get());
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
                    set_login_error.update(|e| *e = None);
                    if res.message.is_some() {
                        set_login_error.update(|e| *e = res.message.clone());
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
                    set_login_error.update(|e| *e = Some(err.to_string()));
                }
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <CredentialsForm
            url=url
            title="Login"
            action_label="Login"
            action=login_action
            error=login_error.into()
            disabled=disabled
            register=false
        />
    }
}
