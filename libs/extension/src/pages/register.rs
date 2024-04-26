use crate::components::credentials::CredentialsForm;
use crate::utils::auth::RegisterCreds;
use leptos::{logging::log, *};

#[component]
pub fn Register() -> impl IntoView {
    let (_register_response, _set_register_response) = create_signal(None::<()>);
    let (register_error, _set_register_error) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let register_action = create_action(move |(email, password): &(String, String)| {
        let email = email.to_string();
        let password = password.to_string();
        let credentials = RegisterCreds {
            email,
            password: password.clone(),
            confirm_password: password.clone(),
        };
        log!("Try to register new account for {}", credentials.email);
        async move {
            set_wait_for_response.update(|w| *w = true);
            // let result = api.register(&credentials).await;
            set_wait_for_response.update(|w| *w = false);
            // match result {
            //     Ok(res) => {
            //         set_register_response.update(|v| *v = Some(res));
            //         set_register_error.update(|e| *e = None);
            //     }
            //     Err(err) => {
            //         // let msg = match err {
            //         //     auth::Error::Fetch(js_err) => {
            //         //         format!("{js_err:?}")
            //         //     }
            //         //     auth::Error::Api(err) => err.message,
            //         // };
            //         log::warn!(
            //             "Unable to register new account for {}: {err}",
            //             credentials.email
            //         );
            //         set_register_error.update(|e| *e = Some(err.to_string()));
            //     }
            // }
        }
    });

    let disabled = Signal::derive(move || wait_for_response.get());

    view! {
        <CredentialsForm
            title="Register"
            action_label="Register"
            action=register_action
            error=register_error.into()
            disabled=disabled
        />
    }
}
