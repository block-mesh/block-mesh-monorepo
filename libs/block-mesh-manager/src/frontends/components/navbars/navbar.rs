use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Navbar(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "flex flex-1 items-center gap-4 py-2.5");

    view! { <nav class=class>{children()}</nav> }
}
