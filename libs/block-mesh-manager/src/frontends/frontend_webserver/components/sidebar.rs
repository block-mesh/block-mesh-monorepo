use leptos::*;
use leptos_router::use_location;
use tailwind_fuse::*;

#[component]
pub fn Sidebar(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "flex h-full min-h-0 flex-col");

    view! { <nav class=class>{children()}</nav> }
}

#[component]
pub fn SidebarHeader(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        tw_join!(class.get(), "flex flex-col border-b border-zinc-950/5 p-4 border-white/5 [&>[data-slot=section]+[data-slot=section]]:mt-2.5")
    };

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn SidebarBody(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        tw_join!(class.get(), "flex flex-1 flex-col overflow-y-auto p-4 [&>[data-slot=section]+[data-slot=section]]:mt-8")
    };

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn SidebarFooter(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || {
        tw_join!(class.get(), "flex flex-col border-t border-zinc-950/5 p-4 border-white/5 [&>[data-slot=section]+[data-slot=section]]:mt-2.5")
    };

    view! { <div class=class>{children()}</div> }
}

#[component]
pub fn SidebarSection(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "flex flex-col gap-0.5");

    view! {
        <div class=class data-slot="section">
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarSpacer(#[prop(into, optional)] class: MaybeSignal<String>) -> impl IntoView {
    let class = move || tw_join!(class.get(), "mt-8 flex-1");

    view! { <div class=class aria-hidden="true"></div> }
}

#[component]
pub fn SidebarItemLink(
    #[prop(into, optional)] class: MaybeSignal<String>,
    #[prop(into)] href: String,
    #[prop(into, optional)] rel: String,
    children: Children,
) -> impl IntoView {
    let pathname = use_location().pathname;

    let current = Signal::derive({
        let href = href.clone();

        move || pathname.get().starts_with(&href)
    });

    let span_class = move || tw_join!(class.get(), "relative");

    let class = move || {
        tw_join!(
            current.get().then_some("text-orange"),
            sidebar_item_classes(),
        )
    };

    view! {
        <span class=span_class>
            <Show when=move || current.get()>
                // Current indicator
                <span class="absolute inset-y-2 -left-4 w-0.5 rounded-full text-off-white bg-off-white"></span>
            </Show>

            <a href=href rel=rel class=class data-current=current>
                {children()}
            </a>
        </span>
    }
}

#[component]
pub fn SidebarItem(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let span_class = move || tw_join!(class.get(), "relative");

    let class = tw_join!("cursor-default", sidebar_item_classes(),);

    view! {
        <span class=span_class>
            <button class=class>{children()}</button>
        </span>
    }
}

#[component]
pub fn SidebarLabel(
    #[prop(into, optional)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let class = move || tw_join!(class.get(), "truncate  text-off-white");

    view! {
        <span class=class data-slot="section">
            {children()}
        </span>
    }
}

fn sidebar_item_classes() -> String {
    tw_join!(
        // Base
        "flex w-full items-center gap-3 rounded-lg px-2 py-2.5 text-left text-base/6 font-medium text-zinc-950 sm:py-2 sm:text-sm/5",
        // Leading icon/icon-only
        "data-[slot=icon]:*:size-6 data-[slot=icon]:*:shrink-0 data-[slot=icon]:*:fill-zinc-500 sm:data-[slot=icon]:*:size-5",
        // Trailing icon (down chevron or similar)
        "data-[slot=icon]:last:*:ml-auto data-[slot=icon]:last:*:size-5 sm:data-[slot=icon]:last:*:size-6",
        // Avatar
        "data-[slot=avatar]:*:-m-0.5 data-[slot=avatar]:*:size-7 data-[slot=avatar]:*:[--ring-opacity:10%] sm:data-[slot=avatar]:*:size-6",
        // Hover
        "data-[hover]:bg-zinc-950/5 data-[slot=icon]:*:data-[hover]:fill-zinc-950",
        // Active
        "data-[active]:bg-zinc-950/5 data-[slot=icon]:*:data-[active]:fill-zinc-950",
        // Current
        "data-[slot=icon]:*:data-[current]:fill-off-white",
        // Dark mode
        "text-off-white data-[slot=icon]:*:fill-zinc-400",
        "data-[hover]:bg-white/5 data-[slot=icon]:*:data-[hover]:fill-white",
        "data-[active]:bg-white/5 data-[slot=icon]:*:data-[active]:fill-white",
        "data-[slot=icon]:*:data-[current]:fill-white",
    )
}
