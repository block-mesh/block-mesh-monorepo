use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn TableHeader(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "border-b border-b-off-white/10 px-4 py-2 font-medium first:pl-[var(--gutter,theme(spacing.2))] last:pr-[var(--gutter,theme(spacing.2))] dark:border-b-off-white/10",
        "sm:first:pl-1 sm:last:pr-1"
    );

    view! { <th class=class>{children()}</th> }
}
