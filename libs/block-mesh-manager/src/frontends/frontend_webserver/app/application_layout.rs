use crate::frontends::components::avatar::Avatar;
use crate::frontends::components::conditionals::if_let_some::IfLetSome;
use crate::frontends::components::icons::home_icon::HomeIcon;
use crate::frontends::components::icons::link_icon::LinkIcon;
use crate::frontends::components::icons::logout_icon::LogoutIcon;
#[allow(unused_imports)]
use crate::frontends::components::icons::medal_icon::MedalIcon;
use crate::frontends::components::icons::perk_icon::PerkIcon;
use crate::frontends::components::navbars::navbar::Navbar;
use crate::frontends::components::navbars::navbar_section::NavbarSection;
use crate::frontends::components::navbars::navbar_spacer::NavbarSpacer;
use crate::frontends::components::reload_button::ReloadButton;
use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::reload_context::ReloadContext;
use crate::frontends::frontend_webserver::components::sidebar::{
    Sidebar, SidebarBody, SidebarFooter, SidebarHeader, SidebarItem, SidebarItemLink, SidebarLabel,
    SidebarSection, SidebarSpacer,
};
use crate::frontends::frontend_webserver::components::sidebar_layout::SidebarLayout;
use crate::frontends::utils::navigate_external::navigate_to_external_url;
use block_mesh_common::constants::BLOCK_MESH_LOGO;
use block_mesh_common::interfaces::server_api::{
    AuthStatusResponse, DailyLeaderboard, DashboardResponse,
};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn ApplicationNavbar() -> impl IntoView {
    view! {
        <Navbar>
            <NavbarSpacer/>
            <NavbarSection>
                <div></div>
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
                    <Avatar src=BLOCK_MESH_LOGO/>
                    <SidebarLabel>BlockMesh</SidebarLabel>
                    <ReloadButton/>
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
                    // <SidebarItemLink href="/ui/daily_leaderboard">
                    //     <MedalIcon/>
                    //     <SidebarLabel>Daily Leaderboard</SidebarLabel>
                    // </SidebarItemLink>
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
pub fn ApplicationLayout(children: ChildrenFn) -> impl IntoView {
    let ReloadContext { value, .. } = expect_context();

    let resource = create_local_resource(
        move || value.get(),
        move |_| async move {
            let origin = window().origin();
            let client = reqwest::Client::new();

            if let Ok(response) = client.get(&format!("{}/auth_status", origin)).send().await {
                if let Ok(json) = response.json::<AuthStatusResponse>().await {
                    provide_context(json);
                } else {
                    navigate_to_external_url(format!("{}/logout", origin));
                }
            } else {
                navigate_to_external_url(format!("{}/logout", origin));
            }

            if let Ok(response) = client.post(&format!("{}/dashboard", origin)).send().await {
                match response.json::<DashboardResponse>().await {
                    Ok(json) => provide_context(json),
                    Err(e) => log!("dashboard json error {:#?}", e),
                }
            }

            if let Ok(response) = client
                .post(&format!(
                    "{}{}",
                    origin,
                    RoutesEnum::Static_Auth_Daily_Leaderboard
                ))
                .send()
                .await
            {
                if let Ok(json) = response.json::<DailyLeaderboard>().await {
                    provide_context(json);
                }
            }
        },
    );

    view! {
        <SidebarLayout navbar=ApplicationNavbar sidebar=ApplicationSidebar>
            <Transition fallback=LoadingIndicator>
                <IfLetSome opt=Signal::derive(move || resource.get()) let:_data clone:children>

                    {children()}

                </IfLetSome>
            </Transition>
        </SidebarLayout>
    }
}

#[component]
pub fn LoadingIndicator() -> impl IntoView {
    view! {
        <div
            class="inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-current border-e-transparent align-[-0.125em] text-surface motion-reduce:animate-[spin_1.5s_linear_infinite] text-off-white"
            role="status"
        ></div>
        <span class="text-off-white !absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]">
            Loading...
        </span>
    }
}
