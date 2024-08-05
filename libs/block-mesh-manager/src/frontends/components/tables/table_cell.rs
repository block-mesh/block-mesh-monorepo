use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn TableCell(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "relative px-4 first:pl-[var(--gutter,theme(spacing.2))] last:pr-[var(--gutter,theme(spacing.2))]",
        "border-b border-zinc-950/5 border-white/5",
        "py-4",
        "sm:first:pl-1 sm:last:pr-1",
    );

    view! { <td class=class>{children()}</td> }
}
