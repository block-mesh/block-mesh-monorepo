use crate::components::sidebar::Sidebar;
use leptos::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <div>
            <Sidebar/>
        </div>
    }
}
