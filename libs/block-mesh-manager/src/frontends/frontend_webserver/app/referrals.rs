#![allow(unexpected_cfgs)]
use crate::frontends::components::edit_invite_code::EditInviteCode;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::info_icon::InfoIcon;
use crate::frontends::components::modal::Modal;
use crate::frontends::components::referer_rank::RefererRank;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::notification_context::NotificationContext;
use block_mesh_common::constants::BUTTON_CLASS;
use block_mesh_common::interfaces::server_api::{DashboardResponse, ReferralSummary};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use reqwest::Client;

#[component]
pub fn Referrals() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let async_data = use_context::<DashboardResponse>();
    let referrals_summary = RwSignal::new(ReferralSummary::default());
    let invite_code = RwSignal::new("".to_string());
    let show_invite_code = RwSignal::new(false);
    let novice = RwSignal::new(false);
    let apprentice = RwSignal::new(false);
    let journeyman = RwSignal::new(false);
    let expert = RwSignal::new(false);
    let master = RwSignal::new(false);
    let grandmaster = RwSignal::new(false);
    let legend = RwSignal::new(false);
    if let Some(data) = async_data {
        novice.set(data.perks.iter().any(|i| i.name == "novice"));
        apprentice.set(data.perks.iter().any(|i| i.name == "apprentice"));
        journeyman.set(data.perks.iter().any(|i| i.name == "journeyman"));
        expert.set(data.perks.iter().any(|i| i.name == "expert"));
        master.set(data.perks.iter().any(|i| i.name == "master"));
        grandmaster.set(data.perks.iter().any(|i| i.name == "grandmaster"));
        legend.set(data.perks.iter().any(|i| i.name == "legend"));
        referrals_summary.set(data.referral_summary);
        invite_code.set(data.invite_code);
    }

    fn get_invite_code() -> Option<String> {
        let doc = document();
        let el = match doc.get_element_by_id("copy_invite_code") {
            None => return None,
            Some(el) => el,
        };
        el.get_attribute("invite_code")
    }

    let apply_ref_action = create_action(move |_| async move {
        let origin = window().origin();
        let client = Client::new();
        let response = client
            .post(format!("{}/api{}", origin, RoutesEnum::Api_ReferralBonus))
            .send()
            .await;
        match response {
            Ok(res) => {
                if res.status().as_u16() == 429 {
                    notifications.set_error("Please wait and retry later");
                } else if res.status().as_u16() != 200 {
                    notifications.set_error("Error Applying Ref Bonus");
                } else {
                    notifications.set_success(
                        "Ref Bonus is being applied in the background, this might take a while",
                    );
                }
            }
            Err(_) => notifications.set_error("Failed Applying Ref Bonus"),
        }
    });

    let apply_ranking_action = create_action(move |_| async move {
        let origin = window().origin();
        let client = Client::new();
        let response = client
            .post(format!("{}/api{}", origin, RoutesEnum::Api_ApplyRanking))
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().as_u16() == 429 {
                    notifications.set_error("Please wait and retry later");
                } else if res.status().as_u16() != 200 {
                    notifications.set_error("Error Applying Rank Bonus");
                } else {
                    notifications.set_success("Ranking Bonus Applied");
                }
            }
            Err(_) => notifications.set_error("Failed Applying Rank Bonus"),
        }
    });

    let copy_to_clipboard = move |_| {
        #[cfg(all(web_sys_unstable_apis, feature = "hydrate"))]
        {
            use crate::frontends::context::notification_context::NotificationContext;
            use leptos_use::{use_clipboard, UseClipboardReturn};

            let notifications = expect_context::<NotificationContext>();
            let UseClipboardReturn { copy, .. } = use_clipboard();

            if let Some(invite_url_string) = get_invite_code() {
                copy(&format!(
                    "https://app.blockmesh.xyz/register?invite_code={}",
                    invite_url_string
                ));
                notifications.set_success("Successfully Copied");
            } else {
                notifications.set_error("Failed to copy invite code");
            }
        }

        #[cfg(not(web_sys_unstable_apis))]
        {}
    };

    view! {
        <Modal show=show_invite_code show_close_button=true>
            <EditInviteCode/>
        </Modal>
        <div class="flex items-start justify-start gap-4">
            <Heading>Referrals</Heading>
            <button
                class=BUTTON_CLASS
                on:click=move |_| apply_ranking_action.dispatch(())
            >
                Apply Ranking Bonus
            </button>
            <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/RANKING_PERK.md"
            >
                <InfoIcon/>
            </a>
            <button
                class=BUTTON_CLASS
                on:click=move |_| apply_ref_action.dispatch(())
            >
                Apply Ref Bonus
            </button>
            <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/REF_BONUS.md"
            >
                <InfoIcon/>
            </a>
            <button
                class=BUTTON_CLASS
                on:click=move |_| {
                    show_invite_code.set(true);
                }
            >

                <span class="material-symbols-outlined">link</span>
                Edit Invite Link
            </button>
            <button
                id="copy_invite_code"
                invite_code=invite_code.get_untracked()
                on:click=copy_to_clipboard
                class="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
            >

                <span class="material-symbols-outlined">link</span>
                Copy Invite Link
            </button>
        </div>

        <div class="referer-ranking my-12">
            <div>
                <Subheading class="mt-14">Ranking</Subheading>
                <nav class="mt-4 mx-auto max-w-7xl" aria-label="Progress">
                    <ol role="list" class="rounded-md xl:flex xl:rounded-none ">
                        <RefererRank
                            title="Novice"
                            description="25 invites"
                            step=1
                            is_complete=novice.get_untracked()
                        />
                        <RefererRank
                            title="Apprentice"
                            description="50 invites"
                            step=2
                            is_complete=apprentice.get_untracked()
                        />
                        <RefererRank
                            title="Journeyman"
                            description="100 invites"
                            step=3
                            is_complete=journeyman.get_untracked()
                        />
                        <RefererRank
                            title="Expert"
                            description="200 invites"
                            step=4
                            is_complete=expert.get_untracked()
                        />
                        <RefererRank
                            title="Master"
                            description="500 invites"
                            step=5
                            is_complete=master.get_untracked()
                        />
                        <RefererRank
                            title="Grandmaster"
                            description="750 invites"
                            step=6
                            is_complete=grandmaster.get_untracked()
                        />
                        <RefererRank
                            title="Legend"
                            description="25,000,000 points and 1,000 invites"
                            step=7
                            is_complete=legend.get_untracked()
                        />
                    </ol>
                </nav>
            </div>

        </div>

        <Subheading class="mt-14">Referrals List</Subheading>
        <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
            <TableHead>
                <tr>
                    <TableHeader>Total Invites</TableHeader>
                    <TableHeader>Total Verified Emails</TableHeader>
                    <TableHeader>Total Verified Humanity</TableHeader>
                    <TableHeader>Total Eligible</TableHeader>
                </tr>
            </TableHead>
            <tbody>
                <tr>
                    <TableCell>{referrals_summary.get_untracked().total_invites}</TableCell>
                    <TableCell>{referrals_summary.get_untracked().total_verified_email}</TableCell>
                    <TableCell>{referrals_summary.get_untracked().total_verified_human}</TableCell>
                    <TableCell>{referrals_summary.get_untracked().total_eligible}</TableCell>
                </tr>
            </tbody>
        </Table>
    }
}
