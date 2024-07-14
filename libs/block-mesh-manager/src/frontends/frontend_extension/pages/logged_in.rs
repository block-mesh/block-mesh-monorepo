use crate::frontends::frontend_extension::extension_state::ExtensionState;
use crate::frontends::frontend_extension::utils::connectors::send_to_clipboard;
use block_mesh_common::constants::BLOCKMESH_VERSION;
use leptos::logging::log;
use leptos::*;
use leptos_router::A;

#[component]
pub fn ExtensionLoggedIn() -> impl IntoView {
    let state = use_context::<ExtensionState>().unwrap();
    let invite_code = Signal::derive(move || state.invite_code.get());
    let invite_url = Signal::derive(move || {
        format!(
            "{}/register?invite_code={}",
            state.blockmesh_url.get(),
            state.invite_code.get()
        )
    });

    let copy_to_clipboard = move |_| {
        spawn_local(async move {
            let invite_url_string = invite_url.get_untracked();
            if invite_code.get_untracked().is_empty() {
                ExtensionState::set_error("Missing invite code".to_string(), state.error);
                return;
            }
            send_to_clipboard(&invite_url_string).await;
            ExtensionState::set_success("Copied to clipboard".to_string(), state.success);
        });
    };

    let logout = create_action(move |_| async move {
        log!("logout");
        state.clear().await;
    });

    view! {
        <div class="auth-card">
            <img
                class="background-image"
                src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/16475f13-7a36-4787-a076-580885250100/public"
                alt="background"
            />
            <div class="auth-card-frame"></div>
            <div class="auth-card-top">
                <div class="auth-card-logout">
                    <svg
                        title="logout"
                        xmlns="http://www.w3.org/2000/svg"
                        height="24px"
                        viewBox="0 -960 960 960"
                        width="24px"
                        fill="#fab457cc"
                        on:click=move |_| {
                            logout.dispatch(());
                        }
                    >

                        <path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h280v80H200v560h280v80H200Zm440-160-55-58 102-102H360v-80h327L585-622l55-58 200 200-200 200Z"></path>
                    </svg>
                </div>
            </div>
            <div class="auth-card-body">
                <img
                    class="h-16 w-16 m-auto"
                    src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                    alt="logo"
                />
                <h1>BlockMesh</h1>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="auth-card-version">Version: {{ BLOCKMESH_VERSION }}</small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>{move || state.email.get()}</strong>
                    </div>
                </div>
            </div>
            <div class="auth-card-bottom logged-bottom">
                <button class="auth-card-button">
                    <A
                        href=move || {
                            let url = state.blockmesh_url.get();
                            format!("{}/ui/dashboard", url)
                        }

                        target="blank"
                    >
                        Dashboard
                    </A>
                </button>
                <button class="auth-card-button" on:click=copy_to_clipboard>
                    Refer
                </button>
            </div>
        </div>
    }
}
