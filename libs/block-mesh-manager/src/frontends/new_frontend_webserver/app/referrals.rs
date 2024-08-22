use crate::frontends::components::edit_invite_code::EditInviteCode;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::modal::Modal;
use crate::frontends::components::referer_rank::RefererRank;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use block_mesh_common::interfaces::server_api::Referral;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn Referrals() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let show_invite_code = create_rw_signal(false);
    let refs: Signal<Vec<Referral>> = Signal::derive(move || {
        if let Some(Some(j)) = async_data.get() {
            j.referrals
        } else {
            vec![]
        }
    });
    fn get_invite_code() -> Option<String> {
        let doc = document();
        let el = match doc.get_element_by_id("copy_invite_code") {
            None => return None,
            Some(el) => el,
        };
        el.get_attribute("invite_code")
    }

    let copy_to_clipboard = move |_| {
        #[cfg(all(web_sys_unstable_apis, feature = "hydrate"))]
        {
            use crate::frontends::context::notification_context::NotificationContext;
            let notifications = expect_context::<NotificationContext>();
            if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
                if let Some(invite_url_string) = get_invite_code() {
                    let _ = clipboard.write_text(&format!(
                        "https://app.blockmesh.xyz/register?invite_code={}",
                        invite_url_string
                    ));
                    notifications.set_success("Successfully Copied");
                } else {
                    notifications.set_error("Failed to copy invite code");
                }
            }
        }
        #[cfg(not(web_sys_unstable_apis))]
        {}
    };

    view! {
        <ApplicationLayout>
            <Modal show=show_invite_code show_close_button=true>
                <EditInviteCode/>
            </Modal>
            <div class="flex items-start justify-start gap-4">
                <Heading>Referrals</Heading>
                <button
                    class="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
                    on:click=move |_| {
                        log!("click");
                        show_invite_code.set(true);
                    }
                >

                    <span class="material-symbols-outlined">link</span>
                    Edit Invite Link
                </button>
                <button
                    id="copy_invite_code"
                    invite_code=move || {
                        match async_data.get() {
                            Some(Some(response)) => response.invite_code.clone(),
                            _ => "".to_string(),
                        }
                    }

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
                                description="100,000 points and 25 invites"
                                step=1
                                is_complete=false
                            />
                            <RefererRank
                                title="Apprentice"
                                description="500,000 points and 50 invites"
                                step=2
                                is_complete=false
                            />
                            <RefererRank
                                title="Journeyman"
                                description="1,000,000 points and 100 invites"
                                step=3
                                is_complete=false
                            />
                            <RefererRank
                                title="Expert"
                                description="2,500,000 points and 200 invites"
                                step=4
                                is_complete=false
                            />
                            <RefererRank
                                title="Master"
                                description="5,000,000 points and 500 invites"
                                step=5
                                is_complete=false
                            />
                            <RefererRank
                                title="Grandmaster"
                                description="10,000,000 points and 750 invites"
                                step=5
                                is_complete=false
                            />
                            <RefererRank
                                title="Legend"
                                description="25,000,000 points and 1,000 invites"
                                step=6
                                is_complete=false
                            />
                        </ol>
                    </nav>
                </div>

            </div>

            <Subheading class="mt-14">Referrals List</Subheading>
            <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
                <TableHead>
                    <tr>
                        <TableHeader>Email</TableHeader>
                        <TableHeader>Joined Date</TableHeader>
                        <TableHeader class="text-right">Verified</TableHeader>
                    </tr>
                </TableHead>
                <tbody>
                    <Suspense>
                        {refs
                            .get()
                            .into_iter()
                            .map(|referral| {
                                view! {
                                    <tr>
                                        <TableCell>{referral.email.clone()}</TableCell>
                                        <TableCell>{referral.created_at.to_string()}</TableCell>
                                        <TableCell class="text-right">
                                            {referral.verified_email.to_string()}
                                        </TableCell>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </Suspense>

                </tbody>
            </Table>
        </ApplicationLayout>
    }
}
