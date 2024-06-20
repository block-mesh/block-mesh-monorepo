use leptos::Suspense;
use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        Loging Page
        </Suspense>
    }
}
