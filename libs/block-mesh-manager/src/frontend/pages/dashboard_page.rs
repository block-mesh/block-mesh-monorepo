use crate::frontend::components::invites::InvitesComponent;
use crate::frontend::components::metrics::MetricsComponent;
use crate::frontend::components::navbar::NavbarComponent;
use crate::frontend::context::webapp_context::WebAppContext;
use leptos::Suspense;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let logged_in = WebAppContext::is_logged_in();

    view! {
        <Suspense fallback=|| {
            view! { <p class="text-white">Loading...</p> }
        }>
            <Show
                when=move || {
                    match logged_in.get() {
                        Some(Some(response)) => response.logged_in,
                        _ => false,
                    }
                }

                fallback=|| {
                    view! { <p class="text-white">Not logged in</p> }
                }
            >

                <NavbarComponent/>
                <MetricsComponent/>
                <InvitesComponent/>
            </Show>
        </Suspense>
    }
}
