use leptos::Suspense;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    logging::log!("Starting");
    view! {
        <Suspense fallback=|| {
            view! {
                <div class="lds-roller"/>
            }
        }>
        HOME PAGe
        </Suspense>
    }
}
