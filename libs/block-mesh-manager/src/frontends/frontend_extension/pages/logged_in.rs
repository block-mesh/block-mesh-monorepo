use leptos::*;

use crate::frontends::frontend_extension::components::notification::Notifications;

#[component]
pub fn ExtensionLoggedIn() -> impl IntoView {
    view! {
        <Notifications/>

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
                    >
                        <path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h280v80H200v560h280v80H200Zm440-160-55-58 102-102H360v-80h327L585-622l55-58 200 200-200 200Z"></path>
                    </svg>
                </div>
            </div>
            <div class="auth-card-body">
                <img
                    class="logo"
                    src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                    alt="logo"
                />
                <h1>BlockMesh</h1>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="auth-card-version">version: 0.0.27</small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>ohaddahan@gmail.com</strong>
                    </div>
                </div>
            </div>
            <div class="auth-card-bottom logged-bottom">
                <button class="auth-card-button">Dashboard</button>
                <button class="auth-card-button">Refer</button>
            </div>
        </div>
    }
}
