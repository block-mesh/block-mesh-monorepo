use crate::frontend::components::invites::InvitesComponent;
use crate::frontend::components::metrics::MetricsComponent;
use crate::frontend::components::navbar::NavbarComponent;
use leptos::Suspense;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <Suspense fallback=|| {
            view! { <div class="lds-roller"></div> }
        }>
            <NavbarComponent/>
            <MetricsComponent/>
            <InvitesComponent/>
        </Suspense>
    }
}
