use crate::components::logo::Logo;
use leptos::*;

#[component]
pub fn ExtensionLoading() -> impl IntoView {
    view! {
        <div class="auth-card">
            <img
                class="background-image"
                src="https://r2-images.blockmesh.xyz/db4411b4-eaf5-45db-121d-15060d780800.png"
                alt="background"
            />
            // <div class="auth-card-frame"></div>
            <div class="auth-card-top"></div>
            <div class="auth-card-body">
                <Logo/>
                <h1 class="text-off-white">BlockMesh</h1>
                <div class="auth-card-content">
                    <div class="pulse"></div>
                    <small class="auth-card-version"></small>
                    <div class="auth-card-chip auth-card-user">
                        <strong>Loading...</strong>
                    </div>
                </div>
            </div>
        </div>
    }
}
