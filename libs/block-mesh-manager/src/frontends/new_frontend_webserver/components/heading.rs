use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-2xl/8 font-semibold text-zinc-950 sm:text-xl/8 dark:text-white">
            {children()}
        </h1>
    }
}

#[component]
pub fn Subheading(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "mt-14 text-base/7 font-semibold text-zinc-950 sm:text-sm/6 dark:text-white"
    );

    view! {
        <h2 class=class>
            {children()}
        </h2>
    }
}
