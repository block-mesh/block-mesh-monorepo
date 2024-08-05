use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn TableHead(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <thead class=tw_join!(& class, "text-off-white text-off-white")>{children()}</thead> }
}
