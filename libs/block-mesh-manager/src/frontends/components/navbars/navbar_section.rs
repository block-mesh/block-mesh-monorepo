use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn NavbarSection(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "flex items-center gap-3");

    view! { <div class=class>{children()}</div> }
}
