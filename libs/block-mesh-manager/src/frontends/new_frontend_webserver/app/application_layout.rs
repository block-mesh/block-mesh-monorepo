use crate::frontends::new_frontend_webserver::components::{
    Avatar, Navbar, NavbarSection, NavbarSpacer, Sidebar, SidebarBody, SidebarFooter,
    SidebarHeader, SidebarItem, SidebarItemLink, SidebarLabel, SidebarLayout, SidebarSection,
    SidebarSpacer,
};
use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn OnlineChip(#[prop(into)] is_online: MaybeSignal<bool>) -> impl IntoView {
    let span_class = move || {
        tw_join!(
            "h-2 w-2 mr-2",
            "rounded-full",
            if is_online.get() {
                "bg-blue shadow-blue"
            } else {
                "bg-darkOrange shadow-darkOrange"
            }
        )
    };

    view! {
        <div class="rounded-lg px-2 flex items-center text-gray-400 ml-auto bg-light">
            <span class=span_class></span>
            <span>{move || if is_online.get() { "Online" } else { "Offline" }}</span>
        </div>
    }
}

#[component]
pub fn ApplicationNavbar() -> impl IntoView {
    view! {
        <Navbar>
            <NavbarSpacer/>

            <NavbarSection>
                <OnlineChip is_online=true/>
            </NavbarSection>
        </Navbar>
    }
}

#[component]
pub fn ApplicationSidebar() -> impl IntoView {
    view! {
        <Sidebar>
            <SidebarHeader>
                <SidebarItem>
                    <Avatar src="/app/logo.png"/>
                    <SidebarLabel>BlockMesh</SidebarLabel>
                    <OnlineChip is_online=true></OnlineChip>
                </SidebarItem>
            </SidebarHeader>

            <SidebarBody>
                <SidebarSection>
                    <SidebarItemLink href="/">
                        <HomeIcon/>
                        <SidebarLabel>Dashboard</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/referer">
                        <LinkIcon/>
                        <SidebarLabel>Referrals</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/extension">
                        <LinkIcon/>
                        <SidebarLabel>Extension</SidebarLabel>
                    </SidebarItemLink>
                </SidebarSection>

                <SidebarSpacer/>

                <SidebarSection>
                    <SidebarItemLink href="logout">
                        <LogoutIcon/>
                        <SidebarLabel>Logout</SidebarLabel>
                    </SidebarItemLink>
                </SidebarSection>
            </SidebarBody>

            <SidebarFooter class="max-lg:hidden">
                <SidebarItem>
                    <span class="flex min-w-0 items-center gap-3">
                        <span class="min-w-0">
                            <span class="block truncate text-sm/5 font-medium text-zinc-950 text-orange">Ohad</span>
                            <span class="block truncate text-xs/5 font-normal text-zinc-500 dark:text-zinc-400">
                              Ohad@blockmesh.com
                            </span>
                        </span>
                    </span>
                </SidebarItem>
            </SidebarFooter>
        </Sidebar>
    }
}

#[component]
pub fn ApplicationLayout(children: Children) -> impl IntoView {
    view! {
        <SidebarLayout
            navbar=ApplicationNavbar
            sidebar=ApplicationSidebar
        >
            {children()}
        </SidebarLayout>
    }
}

#[component]
pub fn LogoutIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true" data-slot="icon" class="cursor-pointer rotate-180">
            <path fill-rule="evenodd" d="M2 4.75A2.75 2.75 0 0 1 4.75 2h3a2.75 2.75 0 0 1 2.75 2.75v.5a.75.75 0 0 1-1.5 0v-.5c0-.69-.56-1.25-1.25-1.25h-3c-.69 0-1.25.56-1.25 1.25v6.5c0 .69.56 1.25 1.25 1.25h3c.69 0 1.25-.56 1.25-1.25v-.5a.75.75 0 0 1 1.5 0v.5A2.75 2.75 0 0 1 7.75 14h-3A2.75 2.75 0 0 1 2 11.25v-6.5Zm9.47.47a.75.75 0 0 1 1.06 0l2.25 2.25a.75.75 0 0 1 0 1.06l-2.25 2.25a.75.75 0 1 1-1.06-1.06l.97-.97H5.25a.75.75 0 0 1 0-1.5h7.19l-.97-.97a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd"/>
        </svg>
    }
}

#[component]
pub fn LinkIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" aria-hidden="true" data-slot="icon">
            <path fill-rule="evenodd" d="M8.914 6.025a.75.75 0 0 1 1.06 0 3.5 3.5 0 0 1 0 4.95l-2 2a3.5 3.5 0 0 1-5.396-4.402.75.75 0 0 1 1.251.827 2 2 0 0 0 3.085 2.514l2-2a2 2 0 0 0 0-2.828.75.75 0 0 1 0-1.06Z" clip-rule="evenodd"></path><path fill-rule="evenodd" d="M7.086 9.975a.75.75 0 0 1-1.06 0 3.5 3.5 0 0 1 0-4.95l2-2a3.5 3.5 0 0 1 5.396 4.402.75.75 0 0 1-1.251-.827 2 2 0 0 0-3.085-2.514l-2 2a2 2 0 0 0 0 2.828.75.75 0 0 1 0 1.06Z" clip-rule="evenodd"/>
        </svg>
    }
}

#[component]
pub fn HomeIcon() -> impl IntoView {
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true" data-slot="icon">
            <path fill-rule="evenodd" d="M9.293 2.293a1 1 0 0 1 1.414 0l7 7A1 1 0 0 1 17 11h-1v6a1 1 0 0 1-1 1h-2a1 1 0 0 1-1-1v-3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1v-6H3a1 1 0 0 1-.707-1.707l7-7Z" clip-rule="evenodd"/>
        </svg>
    }
}
