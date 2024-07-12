use leptos::*;
use leptos_router::A;

use crate::frontends::frontend_extension::components::notification::Notifications;

#[component]
pub fn ExtensionRegister() -> impl IntoView {
    let _register_action = create_action(move |_params: &Vec<String>| {
        async move {}
        // let email = params[0].to_string();
        // let password = params[1].to_string();
        // let password_confirm = params[2].to_string();
        // let invite_code = params[3].to_string();
        // let credentials = RegisterForm {
        //     email,
        //     password,
        //     password_confirm,
        //     invite_code: if invite_code.is_empty() {
        //         None
        //     } else {
        //         Some(invite_code)
        //     },
        // };
        // log!("Try to register new account for {}", credentials.email);
        // async move {
        //     if credentials.password != credentials.password_confirm {
        //         tracing::warn!("Passwords do not match");
        //         AppState::set_error("Passwords do not match".to_string(), state.error);
        //         return;
        //     }
        //     set_wait_for_response.update(|w| *w = true);
        //     let result = register(&state.blockmesh_url.get_untracked(), &credentials).await;
        //     set_wait_for_response.update(|w| *w = false);
        //     match result {
        //         Ok(_) => {
        //             state.api_token.update(|t| *t = uuid::Uuid::default());
        //             AppState::store_api_token(uuid::Uuid::default()).await;
        //             state
        //                 .status
        //                 .update(|v| *v = AppStatus::WaitingEmailVerification);
        //             on_success.call(());
        //         }
        //         Err(err) => {
        //             tracing::error!(
        //                 "Unable to register new account for {}: {err}",
        //                 credentials.email
        //             );
        //             AppState::set_error(err.to_string(), state.error);
        //         }
        //     }
        // }
    });

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
                    <A href="/ext/login">Login now</A>
                </small>
            </div>
        </div>
    }
}
