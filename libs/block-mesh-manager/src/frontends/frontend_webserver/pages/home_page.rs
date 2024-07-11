use leptos::Suspense;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! { <div class="lds-roller"></div> }
        }>HOME page</Suspense>
    }
}
