use crate::frontends::new_frontend_webserver::components::{
    Navbar, NavbarSection, NavbarSpacer, Sidebar, SidebarBody, SidebarHeader, SidebarItem,
    SidebarLabel, SidebarLayout, SidebarSection,
};
use leptos::*;
use leptos_router::use_location;
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
                <Dropdown>
                    <DropdownButton as={SidebarItem}>
                        <Avatar src="/app/logo.png"/>
                        <SidebarLabel>BlockMesh</SidebarLabel>
                        <OnlineChip class="desktop-online-chip" is_online=true></OnlineChip>
                    </DropdownButton>
                </Dropdown>
            </SidebarHeader>

            <SidebarBody>
                <SidebarSection>
                    <SidebarItem href="/">
                        <HomeIcon/>
                        <SidebarLabel>Dashboard</SidebarLabel>
                    </SidebarItem>
                    <SidebarItem href="/referer">
                        <LinkIcon/>
                        <SidebarLabel>Referrals</SidebarLabel>
                    </SidebarItem>
                    <SidebarItem href="/extension">
                        <LinkIcon/>
                        <SidebarLabel>Extension</SidebarLabel>
                    </SidebarItem>
                </SidebarSection>

                <SidebarSpacer/>

                <SidebarSection>
                    <SidebarItem href="logout">
                        <ArrowRightStartOnRectangleIcon/>
                        <SidebarLabel>Logout</SidebarLabel>
                    </SidebarItem>
                </SidebarSection>
            </SidebarBody>

            <SidebarFooter class="max-lg:hidden">
                <Dropdown>
                    <DropdownButton as={SidebarItem}>
        <span class="flex min-w-0 items-center gap-3">
          <span class="min-w-0">
            <span class="block truncate text-sm/5 font-medium text-zinc-950 text-orange">Ohad</span>
            <span class="block truncate text-xs/5 font-normal text-zinc-500 dark:text-zinc-400">
              Ohad@blockmesh.com
            </span>
          </span>
        </span>
                    </DropdownButton>
                </Dropdown>
            </SidebarFooter>
        </Sidebar>
    }
}

#[component]
pub fn ApplicationLayout(children: Children) -> impl IntoView {
    let pathname = use_location().pathname;

    view! {
        <SidebarLayout
            navbar=ApplicationNavbar
            sidebar=ApplicationSidebar
        >
    }
}
