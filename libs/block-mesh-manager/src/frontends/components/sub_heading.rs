use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Subheading(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "mt-14 text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white"
    );

    view! { <h2 class=class>{children()}</h2> }
}
