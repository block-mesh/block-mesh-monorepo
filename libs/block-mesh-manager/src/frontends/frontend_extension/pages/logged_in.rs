use leptos::*;
use leptos_router::A;

use crate::frontends::context::extension_state::ExtensionContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::frontend_extension::components::logo::Logo;
use crate::frontends::utils::connectors::send_to_clipboard;
use block_mesh_common::constants::BLOCKMESH_VERSION;

#[component]
pub fn ExtensionLoggedIn() -> impl IntoView {
    let state = use_context::<ExtensionContext>().unwrap();
    let notifications = expect_context::<NotificationContext>();
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
                notifications.set_error(
                    "Missing invite code, please close and reopen the extension popup".to_string(),
                );
                return;
            }
            send_to_clipboard(&invite_url_string).await;
            notifications.set_success("Copied to clipboard".to_string());
        });
    };

    let logout = create_action(move |_| async move {
        state.clear().await;
    });

    view! {
        <div class="auth-card">
            <img
                class="background-image"
                src="https://r2-images.blockmesh.xyz/dc54851e-a585-44af-e6b6-18d16c984500.png"
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
                        fill="#FF7E07"
                        on:click=move |_| {
                            logout.dispatch(());
                        }
                    >

                        <path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h280v80H200v560h280v80H200Zm440-160-55-58 102-102H360v-80h327L585-622l55-58 200 200-200 200Z"></path>
                    </svg>
                </div>
            </div>
            <div class="auth-card-body">
                <Logo/>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="relative text-off-white">Version: {{ BLOCKMESH_VERSION }}</small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>{move || state.email.get()}</strong>
                    </div>
                </div>
            </div>
            <div class="auth-card-bottom logged-bottom">
                <button class="auth-card-button font-bebas-neue text-off-white">
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
                <button
                    class="auth-card-button font-bebas-neue text-off-white"
                    on:click=copy_to_clipboard
                >
                    Refer
                </button>
            </div>
        </div>
    }
}
