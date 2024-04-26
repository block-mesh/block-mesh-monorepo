use crate::components::credentials::CredentialsForm;
use crate::utils::auth::{login, LoginCreds};
use crate::utils::log::log;
use crate::utils::state::AppState;
use leptos::*;

#[component]
pub fn Login() -> impl IntoView {
    let (login_error, set_login_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);
    let state = use_context::<AppState>().unwrap();
    let login_action = create_action(move |(email, password): &(String, String)| {
        log::debug!("Try to login with {email}");
        let email = email.to_string();
        let password = password.to_string();
        let credentials = LoginCreds { email, password };
        async move {
            set_wait_for_response.update(|w| *w = true);
            let result = login(&state.blockmesh_url.get_untracked(), &credentials).await;
            set_wait_for_response.update(|w| *w = false);
            match result {
                Ok(res) => {
                    set_login_error.update(|e| *e = None);
                    if res.api_token != state.api_token.get_untracked() {
                        log::debug!("Store new api token");
                        AppState::store_api_token(res.api_token).await;
                    }
                }
                Err(err) => {
                    log::error!("Unable to login with {}: {err}", credentials.email);
                    set_login_error.update(|e| *e = Some(err.to_string()));
                }
            }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <CredentialsForm
            title="Login"
            action_label="Login"
            action=login_action
            error=login_error.into()
            disabled=disabled
        />
    }
}
