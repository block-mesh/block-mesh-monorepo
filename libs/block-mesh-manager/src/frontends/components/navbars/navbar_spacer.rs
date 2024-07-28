use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn NavbarSpacer(#[prop(into, optional)] class: MaybeSignal<String>) -> impl IntoView {
    let class = move || tw_join!(class.get(), "-ml-4 flex-1");

    view! { <div class=class aria-hidden="true"></div> }
}
