use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Subheading(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "font-bebas-neue mt-14 text-base/7 font-semibold text-off-white sm:text-sm/6 text-off-white"
    );

    view! { <h2 class=class>{children()}</h2> }
}
