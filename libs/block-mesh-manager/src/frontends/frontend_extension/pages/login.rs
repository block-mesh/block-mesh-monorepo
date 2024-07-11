use block_mesh_common::interfaces::server_api::{GetTokenResponse, LoginForm};
use leptos::logging::log;
use leptos::*;
use leptos_router::{use_navigate, A};

#[tracing::instrument(name = "login", skip(credentials), err)]
pub async fn login(
    blockmesh_url: &str,
    credentials: &LoginForm,
) -> anyhow::Result<GetTokenResponse> {
    let url = format!("{}/api/get_token", blockmesh_url);
    let client = reqwest::Client::new();
    log!("pre");
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&credentials)
        .send()
        .await?
        .json()
        .await?;
    log!("post response {:?}", response);
    Ok(response)
}

#[component]
pub fn ExtensionLogin() -> impl IntoView {
    let (password, set_password) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let url = "http://localhost:8000";

    let submit_action = create_action(move |_| async move {
        // send_message("{ hello: \"world\"").await;

        log!("bla");
        let credentials = LoginForm {
            email: email.get_untracked(),
            password: password.get_untracked(),
        };

        log!("credentials {:?}", credentials);
        let result = login(url, &credentials).await;
        log!("result {:?}", result);
        match result {
            Ok(res) => {
                // state.email.update(|e| *e = credentials.email.clone());
                // AppState::store_email(credentials.email).await;
                if res.message.is_some() {
                    // AppState::set_error(res.message.unwrap(), state.error);
                    return;
                }
                if let Some(_api_token) = res.api_token {
                    // if api_token != state.api_token.get_untracked()
                    //     || state.api_token.get_untracked() == Uuid::default()
                    // {
                    //     tracing::info!("Store new api token");
                    //     AppState::store_api_token(api_token).await;
                    // } else {
                    //     tracing::info!("Logged in");
                    // }
                    // state.status.update(|v| *v = AppStatus::LoggedIn);
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
    });

    view! {
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
                <form  on:submit=|ev| ev.prevent_default()>
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
                                    "Enter" => submit_action.dispatch(()),
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
                    <button class="auth-card-button" on:click=move |_ev| submit_action.dispatch(())>
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
