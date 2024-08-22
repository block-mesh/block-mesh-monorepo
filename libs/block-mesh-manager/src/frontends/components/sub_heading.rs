use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Subheading(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(&class, "font-bebas-neue text-off-white");

    view! { <h2 class=class>{children()}</h2> }
}
