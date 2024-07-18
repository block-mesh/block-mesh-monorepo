use crate::frontends::new_frontend_webserver::components::NavbarItem;
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn OpenMenuIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <path d="M2 6.75C2 6.33579 2.33579 6 2.75 6H17.25C17.6642 6 18 6.33579 18 6.75C18 7.16421 17.6642 7.5 17.25 7.5H2.75C2.33579 7.5 2 7.16421 2 6.75ZM2 13.25C2 12.8358 2.33579 12.5 2.75 12.5H17.25C17.6642 12.5 18 12.8358 18 13.25C18 13.6642 17.6642 14 17.25 14H2.75C2.33579 14 2 13.6642 2 13.25Z" />
        </svg>
    }
}

#[component]
pub fn CloseMenuIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 20 20" aria-hidden="true">
          <path d="M6.28 5.22a.75.75 0 0 0-1.06 1.06L8.94 10l-3.72 3.72a.75.75 0 1 0 1.06 1.06L10 11.06l3.72 3.72a.75.75 0 1 0 1.06-1.06L11.06 10l3.72-3.72a.75.75 0 0 0-1.06-1.06L10 8.94 6.28 5.22Z" />
        </svg>
    }
}

#[component]
pub fn MobileSidebar<CloseFn>(
    #[prop(into)] open: Signal<bool>,
    on_close: CloseFn,
    children: Children,
) -> impl IntoView
where
    CloseFn: Fn() + Clone + 'static,
{
    let aside_class =
        Signal::derive(move || tw_join!("lg:hidden z-20", (!open.get()).then(|| "hidden")));
    let backdrop_class = move || {
        tw_join!(
            "fixed inset-0 bg-black/30 z-10",
            "w-screen h-screen",
            "transition data-[closed]:opacity-0 data-[enter]:duration-300 data-[leave]:duration-200 data-[enter]:ease-out data-[leave]:ease-in",
            aside_class.get()
        )
    };

    view! {
        <aside class=aside_class role="dialog" aria-modal="true">
            <div class="fixed inset-y-0 w-full max-w-80 p-2 transition duration-300 ease-in-out data-[closed]:-translate-x-full">
            </div>
        </aside>

        <div class=backdrop_class aria-hidden="true" on:click=on_close>
            <div class="flex h-full flex-col rounded-lg bg-white shadow-sm ring-1 ring-zinc-950/5 dark:bg-zinc-900 dark:ring-white/10">
                <div class="-mb-3 px-4 pt-3">
                    <span class="relative">
                        <button class="cursor-default relative flex min-w-0 items-center gap-3 rounded-lg p-2 text-left text-base/6 font-medium text-zinc-950 sm:text-sm/5 data-[slot=icon]:*:size-6 data-[slot=icon]:*:shrink-0 data-[slot=icon]:*:fill-zinc-500 sm:data-[slot=icon]:*:size-5 data-[slot=icon]:last:[&:not(:nth-child(2))]:*:ml-auto data-[slot=icon]:last:[&:not(:nth-child(2))]:*:size-5 sm:data-[slot=icon]:last:[&:not(:nth-child(2))]:*:size-4 data-[slot=avatar]:*:-m-0.5 data-[slot=avatar]:*:size-7 data-[slot=avatar]:*:[--avatar-radius:theme(borderRadius.DEFAULT)] data-[slot=avatar]:*:[--ring-opacity:10%] sm:data-[slot=avatar]:*:size-6 data-[hover]:bg-zinc-950/5 data-[slot=icon]:*:data-[hover]:fill-zinc-950 data-[active]:bg-zinc-950/5 data-[slot=icon]:*:data-[active]:fill-zinc-950 dark:text-white dark:data-[slot=icon]:*:fill-zinc-400 dark:data-[hover]:bg-white/5 dark:data-[slot=icon]:*:data-[hover]:fill-white dark:data-[active]:bg-white/5 dark:data-[slot=icon]:*:data-[active]:fill-white" aria-label="Close navigation" type="button" on:click=on_close.clone()>
                            <CloseMenuIcon />
                        </button>
                    </span>
                </div>

                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn SidebarLayout<NavbarFn, SidebarFn, V1, V2>(
    navbar: NavbarFn,
    sidebar: SidebarFn,
    children: Children,
) -> impl IntoView
where
    NavbarFn: Fn() -> V1 + Clone + 'static,
    SidebarFn: Fn() -> V2 + Clone + 'static,
    V1: IntoView,
    V2: IntoView,
{
    let (show_sidebar, set_show_sidebar) = create_signal(false);

    view! {
        <div class="relative isolate flex min-h-svh w-full bg-white max-lg:flex-col lg:bg-zinc-100 dark:bg-zinc-900 dark:lg:bg-zinc-950">
            // Sidebar on desktop
            <div class="fixed inset-y-0 left-0 w-64 max-lg:hidden">{sidebar}</div>

            // Sidebar on mobile
            <MobileSidebar
                open=show_sidebar
                on_close=move || set_show_sidebar.set(false)
            >
                {sidebar}
            </MobileSidebar>

            // Navbar on mobile
            <header class="flex items-center px-4 lg:hidden">
                <div class="py-2.5">
                    <NavbarItem on:click=move |_| set_show_sidebar.set(true) aria-label="Open navigation">
                        <OpenMenuIcon />
                    </NavbarItem>
                </div>
                <div class="min-w-0 flex-1">{navbar}</div>
            </header>

            // Content
            <main class="flex flex-1 flex-col pb-2 lg:min-w-0 lg:pl-64 lg:pr-2 lg:pt-2">
                <div
                    class="page-wrapper grow p-6 lg:rounded-lg lg:bg-white lg:p-10 lg:shadow-sm lg:ring-1 lg:ring-zinc-950/5 dark:lg:bg-zinc-900 dark:lg:ring-white/10">
                    <div class="mx-auto max-w-6xl">{children()}</div>
                </div>
            </main>

        </div>
    }
}
