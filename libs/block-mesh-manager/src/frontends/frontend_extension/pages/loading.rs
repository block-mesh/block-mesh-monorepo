use leptos::*;

#[component]
pub fn ExtensionLoading() -> impl IntoView {
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
                <div class="flex justify-center">
                    <img
                        class="h-16 w-16 m-auto"
                        src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                        alt="logo"
                    />
                </div>
                <h1>BlockMesh</h1>
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
