use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn NavbarLabel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "truncate");

    view! { <span class=class>{children()}</span> }
}
