use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn TableHead(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <thead class=tw_join!(& class, "text-zinc-500 dark:text-zinc-400")>{children()}</thead> }
}
