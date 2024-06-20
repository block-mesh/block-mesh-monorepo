use leptos::Suspense;
use leptos::*;

#[component]
pub fn NewPasswordPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        New Password Page
        </Suspense>
    }
}
