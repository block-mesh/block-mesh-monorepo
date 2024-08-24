use crate::frontends::components::avatar::Avatar;
use crate::frontends::components::navbars::navbar::Navbar;
use crate::frontends::components::navbars::navbar_section::NavbarSection;
use crate::frontends::components::navbars::navbar_spacer::NavbarSpacer;
// use crate::frontends::components::online_chip::OnlineChip;
use crate::frontends::components::conditionals::if_let_some::IfLetSome;
use crate::frontends::components::icons::home_icon::HomeIcon;
use crate::frontends::components::icons::link_icon::LinkIcon;
use crate::frontends::components::icons::logout_icon::LogoutIcon;
use crate::frontends::components::icons::perk_icon::PerkIcon;
use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::new_frontend_webserver::components::sidebar::{
    Sidebar, SidebarBody, SidebarFooter, SidebarHeader, SidebarItem, SidebarItemLink, SidebarLabel,
    SidebarSection, SidebarSpacer,
};
use crate::frontends::new_frontend_webserver::components::sidebar_layout::SidebarLayout;
use block_mesh_common::constants::BLOCK_MESH_LOGO;
use block_mesh_common::interfaces::server_api::DashboardResponse;
use leptos::*;

#[component]
pub fn ApplicationNavbar() -> impl IntoView {
    view! {
        <Navbar>
            <NavbarSpacer />
            <NavbarSection>
                <div></div>
            // <OnlineChip is_online=true/>
            </NavbarSection>
        </Navbar>
    }
}

#[component]
pub fn ApplicationSidebar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let email = auth.email;

    view! {
        <Sidebar>
            <SidebarHeader>
                <SidebarItem>
                    <Avatar src=BLOCK_MESH_LOGO />
                    <SidebarLabel>BlockMesh</SidebarLabel>
                // <OnlineChip is_online=true/>
                </SidebarItem>
            </SidebarHeader>

            <SidebarBody>
                <SidebarSection>
                    <SidebarItemLink href="/ui/dashboard">
                        <HomeIcon />
                        <SidebarLabel>Dashboard</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/referrals">
                        <LinkIcon />
                        <SidebarLabel>Referrals</SidebarLabel>
                    </SidebarItemLink>
                    <SidebarItemLink href="/ui/perks">
                        <PerkIcon />
                        <SidebarLabel>Perks</SidebarLabel>
                    </SidebarItemLink>
                </SidebarSection>

                <SidebarSpacer />

                <SidebarSection>
                    <SidebarItemLink href="/logout" rel="external">
                        <LogoutIcon />
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
pub fn ApplicationLayout(children: ChildrenFn) -> impl IntoView {
    let resource = create_local_resource(
        move || (),
        move |_| async move {
            let origin = window().origin();
            let client = reqwest::Client::new();
            let response = client.post(&format!("{}/dashboard", origin)).send().await;

            match response {
                Ok(response) => match response.json::<DashboardResponse>().await {
                    Ok(json) => Some(json),
                    Err(e) => {
                        logging::log!("error: {}", e);
                        None
                    }
                },
                Err(e) => {
                    logging::log!("error: {}", e);
                    None
                }
            }
        },
    );

    view! {
        <SidebarLayout navbar=ApplicationNavbar sidebar=ApplicationSidebar>
            <Transition fallback=move || view! { <p>TODO: Loading...</p> }.into_view()>
                <IfLetSome opt=Signal::derive(move || resource.get().flatten()) let:data clone:children>
                    {
                        provide_context(data.clone());
                        children()
                    }
                </IfLetSome>
            </Transition>
        </SidebarLayout>
    }
}
