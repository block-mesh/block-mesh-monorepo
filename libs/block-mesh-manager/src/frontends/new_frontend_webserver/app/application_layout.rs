use crate::frontends::components::avatar::Avatar;
use crate::frontends::components::navbars::navbar::Navbar;
use crate::frontends::components::navbars::navbar_section::NavbarSection;
use crate::frontends::components::navbars::navbar_spacer::NavbarSpacer;
// use crate::frontends::components::online_chip::OnlineChip;
use crate::frontends::components::icons::home_icon::HomeIcon;
use crate::frontends::components::icons::link_icon::LinkIcon;
use crate::frontends::components::icons::logout_icon::LogoutIcon;
use crate::frontends::components::icons::medal_icon::MedalIcon;
use crate::frontends::components::icons::perk_icon::PerkIcon;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::new_frontend_webserver::components::sidebar::{
    Sidebar, SidebarBody, SidebarFooter, SidebarHeader, SidebarItem, SidebarItemLink, SidebarLabel,
    SidebarSection, SidebarSpacer,
};
use crate::frontends::new_frontend_webserver::components::sidebar_layout::SidebarLayout;
use block_mesh_common::constants::BLOCK_MESH_LOGO;
use leptos::*;

#[component]
pub fn ApplicationNavbar() -> impl IntoView {
    view! {
        <Navbar>
            <NavbarSpacer/>
            <NavbarSection>
                <div></div>
            // <OnlineChip is_online=true/>
            </NavbarSection>
        </Navbar>
    }
}

#[component]
pub fn ApplicationSidebar() -> impl IntoView {
    let logged_in = WebAppContext::is_logged_in();
    let email = Signal::derive(move || {
        if let Some(Some(r)) = logged_in.get() {
            r.email
        } else {
            None
        }
    });
    view! {
        <Sidebar>
            <SidebarHeader>
                <SidebarItem>
                    <Avatar src=BLOCK_MESH_LOGO/>
                    <SidebarLabel>BlockMesh</SidebarLabel>
                // <OnlineChip is_online=true/>
                </SidebarItem>
            </SidebarHeader>

            <SidebarBody>
                <SidebarSection>
                    <SidebarItemLink href="/ui/dashboard">
                        <HomeIcon/>
                        <SidebarLabel>Dashboard</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/referrals">
                        <LinkIcon/>
                        <SidebarLabel>Referrals</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/perks">
                        <PerkIcon/>
                        <SidebarLabel>Perks</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/daily_leaderboard">
                        <MedalIcon/>
                        <SidebarLabel>Daily Leaderboard</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/admin_dashboard">
                        <HomeIcon/>
                        <SidebarLabel>Admin Dashboard</SidebarLabel>
                    </SidebarItemLink>
                </SidebarSection>

                <SidebarSpacer/>

                <SidebarSection>
                    <SidebarItemLink href="/logout" rel="external">
                        <LogoutIcon/>
                        <SidebarLabel>Logout</SidebarLabel>
                    </SidebarItemLink>
                </SidebarSection>
            </SidebarBody>

            <SidebarFooter class="max-lg:hidden">
                <SidebarItem>
                    <span class="flex min-w-0 items-center gap-3">
                        <span class="min-w-0">
                            <span class="block truncate text-sm/5 font-medium text-zinc-950 text-orange"></span>
                            <span class="block truncate text-xs/5 font-normal text-zinc-500 text-zinc-400">
                                {move || email.get()}
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
        <SidebarLayout navbar=ApplicationNavbar sidebar=ApplicationSidebar>
            {children()}
        </SidebarLayout>
    }
}
