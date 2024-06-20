use leptos::Suspense;
use leptos::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        Register Page
        </Suspense>
    }
}
