use crate::frontends::components::icons::info_icon::InfoIcon;
use crate::frontends::components::icons::twitter_icon::TwitterIcon;
use crate::frontends::frontend_webserver::app::perks_data::show_perk;
use block_mesh_common::constants::{
    BIT_ROBOT_TWITTER_ID, BLOCKMESH_FOUNDER_TWITTER_USER_ID, BLOCKMESH_TWITTER_USER_ID,
    BUTTON_CLASS, FRODOBOTS_TWITTER_ID, ROBOTS_DOT_FUN_ID, SAM_IS_MOVING_TWITTER_ID,
    UFBOTS_TWITTER_ID,
};
use block_mesh_common::interfaces::server_api::DashboardResponse;
use leptos::*;

#[component]
pub fn PerksModal() -> impl IntoView {
    let perks = RwSignal::new(vec![]);
    let async_data = use_context::<DashboardResponse>();
    if let Some(data) = async_data {
        perks.set(data.perks);
    }
    view! {
        <div class="bg-dark-blue">
            <form action="/edit_invite_code" method="post" on:submit=|ev| ev.prevent_default()>
                <div class="m-2">
                    <div class="mb-4 rounded px-8 pb-8 pt-6 shadow-md bg-dark-blue">
                        <div class="flex flex-col gap-4">
                            <div>
                                <h2 class="text-2xl font-bold text-off-white mb-4">Instructions</h2>
                                <ol class="list-decimal list-inside space-y-2 text-off-white">
                                    <li>Click on the link to the profile on Twitter</li>
                                    <li>Follow the account</li>
                                    <li>Come back here</li>
                                    <li>Verify</li>
                                </ol>
                            </div>
                            <hr class="border-t border-white mb-4" />
                            <Show when=move || { show_perk(&perks.get(), "twitter") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!(
                                            "/twitter/login?target={}",
                                            BLOCKMESH_TWITTER_USER_ID,
                                        )

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "twitter") {
                                                "Twitter Connected"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/blockmesh_xyz"
                                        target="_blank"
                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "twitter") {
                                                "Twitter Connected"
                                            } else {
                                                "@blockmesh_xyz"
                                            }
                                        }}

                                    </a>

                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "founder_twitter") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!(
                                            "/twitter/login?target={}",
                                            BLOCKMESH_FOUNDER_TWITTER_USER_ID,
                                        )

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "founder_twitter") {
                                                "Founder Followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/__OhadDahan__"
                                        target="_blank"
                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "founder_twitter") {
                                                "Founder Followed"
                                            } else {
                                                "@__OhadDahan__"
                                            }
                                        }}

                                    </a>
                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "UFBots") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!("/twitter/login?target={}", UFBOTS_TWITTER_ID)

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "UFBots") {
                                                "UFBots followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/UFBots"
                                        target="_blank"

                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "UFBots") {
                                                "UFBots followed"
                                            } else {
                                                "@UFBots"
                                            }
                                        }}

                                    </a>

                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "FrodoBots") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!(
                                            "/twitter/login?target={}",
                                            FRODOBOTS_TWITTER_ID,
                                        )

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "FrodoBots") {
                                                "FrodoBots followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/frodobots"
                                        target="_blank"

                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "FrodoBots") {
                                                "FrodoBots followed"
                                            } else {
                                                "@FrodoBots"
                                            }
                                        }}

                                    </a>

                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "SamIsMoving") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!(
                                            "/twitter/login?target={}",
                                            SAM_IS_MOVING_TWITTER_ID,
                                        )

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "SamIsMoving") {
                                                "SamIsMoving followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/SamIsMoving"
                                        target="_blank"

                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "SamIsMoving") {
                                                "SamIsMoving followed"
                                            } else {
                                                "@SamIsMoving"
                                            }
                                        }}

                                    </a>

                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "BitRobotNetwork") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!(
                                            "/twitter/login?target={}",
                                            BIT_ROBOT_TWITTER_ID,
                                        )

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "BitRobotNetwork") {
                                                "BitRobotNetwork followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/BitRobotNetwork"
                                        target="_blank"
                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "BitRobotNetwork") {
                                                "BitRobotNetwork followed"
                                            } else {
                                                "@BitRobotNetwork"
                                            }
                                        }}

                                    </a>
                                </div>
                            </Show>
                            <Show when=move || { show_perk(&perks.get(), "RobotsDotFun") }>
                                <div class="flex flex-row gap-4">
                                    <a
                                        rel="external"
                                        target="_blank"
                                        href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
                                    >
                                        <InfoIcon/>
                                    </a>
                                    <a
                                        rel="external"
                                        href=format!("/twitter/login?target={}", ROBOTS_DOT_FUN_ID)

                                        class=BUTTON_CLASS
                                    >
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "RobotsDotFun") {
                                                "RobotsDotFun followed"
                                            } else {
                                                "Verify"
                                            }
                                        }}

                                    </a>
                                    <a
                                        rel="external"
                                        href="https://x.com/robotsdotfun"
                                        target="_blank"
                                        class=BUTTON_CLASS
                                    >
                                        <TwitterIcon/>
                                        {move || {
                                            if perks.get().iter().any(|i| i.name == "RobotsDotFun") {
                                                "RobotsDotFun followed"
                                            } else {
                                                "@RobotsDotFun"
                                            }
                                        }}

                                    </a>
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
            </form>
        </div>
    }
}
