use crate::frontends::components::heading::Heading;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use leptos::*;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <ApplicationLayout>
            <div class="flex items-start justify-start gap-4">
                <Heading>Admin Dashboard</Heading>
            </div>
        </ApplicationLayout>
    }
}
