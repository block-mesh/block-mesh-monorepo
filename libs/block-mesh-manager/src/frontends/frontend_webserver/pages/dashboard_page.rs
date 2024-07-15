use crate::frontends::frontend_webserver::components::error::ErrorComponent;
use crate::frontends::frontend_webserver::components::invites::InvitesComponent;
use crate::frontends::frontend_webserver::components::metrics::MetricsComponent;
use crate::frontends::frontend_webserver::components::navbar::NavbarComponent;
use crate::frontends::frontend_webserver::components::notification::NotificationComponent;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use leptos::Suspense;
use leptos::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let logged_in = WebAppContext::is_logged_in();
    view! {
        <Suspense fallback=|| {
            view! {
                <NotificationComponent
                    summary="Loading...".to_string()
                    detailed="Please wait while we load the dashboard".to_string()
                    go_to="/ui/login".to_string()
                />
            }
        }>
            <Show
                when=move || {
                    match logged_in.get() {
                        Some(Some(response)) => response.logged_in,
                        _ => false,
                    }
                }

                fallback=|| {
                    view! {
                        <ErrorComponent
                            code=401
                            summary="Not Logged In".to_string()
                            detailed="You must be logged in to view this page".to_string()
                            go_to="/ui/login".to_string()
                        />
                    }
                }
            >

                <div class="bg-dark-blue h-screen">
                    <NavbarComponent/>
                    <MetricsComponent/>
                    <InvitesComponent/>
                </div>
            </Show>
        </Suspense>
    }
}
