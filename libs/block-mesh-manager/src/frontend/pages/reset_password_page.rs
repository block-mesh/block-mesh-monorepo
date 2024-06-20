use leptos::Suspense;
use leptos::*;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        Reset Password Page
        </Suspense>
    }
}
