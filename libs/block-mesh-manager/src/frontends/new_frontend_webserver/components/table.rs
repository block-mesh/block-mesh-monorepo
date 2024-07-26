use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Table(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div className="flow-root">
            <div class=tw_join!(& class, "-mx-[--gutter] overflow-x-auto whitespace-nowrap")>
                <div class=tw_join!("inline-block min-w-full align-middle", "sm:px-[--gutter]")>
                    <table class="min-w-full text-left text-sm/6 text-zinc-950 dark:text-white">
                        {children()}
                    </table>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TableHead(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! { <thead class=tw_join!(& class, "text-zinc-500 dark:text-zinc-400")>{children()}</thead> }
}

#[component]
pub fn TableHeader(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "border-b border-b-zinc-950/10 px-4 py-2 font-medium first:pl-[var(--gutter,theme(spacing.2))] last:pr-[var(--gutter,theme(spacing.2))] dark:border-b-white/10",
        "sm:first:pl-1 sm:last:pr-1"
    );

    view! {
        <th class=class>{children()}</th>
    }
}

#[component]
pub fn TableCell(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let class = tw_join!(
        &class,
        "relative px-4 first:pl-[var(--gutter,theme(spacing.2))] last:pr-[var(--gutter,theme(spacing.2))]",
        "border-b border-zinc-950/5 dark:border-white/5",
        "sm:first:pl-1 sm:last:pr-1",
    );

    view! {
        <td class=class>{children()}</td>
    }
}
