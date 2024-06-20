use leptos::Suspense;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        Dashboard Page
        </Suspense>
    }
}
