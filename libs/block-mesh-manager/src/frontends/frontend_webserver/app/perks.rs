use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::twitter_icon::TwitterIcon;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::utils::auth::connect_wallet_in_browser;
use block_mesh_common::interfaces::server_api::DashboardResponse;
use leptos::*;
use leptos_use::js;

#[component]
pub fn Perks() -> impl IntoView {
    let async_data = use_context::<DashboardResponse>();
    let notifications = expect_context::<NotificationContext>();
    let auth = expect_context::<AuthContext>();
    let perks = RwSignal::new(vec![]);
    if let Some(data) = async_data {
        perks.set(data.perks);
    }

    let has_backpack = RwSignal::new(false);

    create_effect(move |_| {
        has_backpack.set(js!("backpack" in &window()));
    });

    let button_enabled = Signal::derive(move || auth.wallet_address.get().is_none());

    let on_connect_button_click = move || {
        spawn_local(async move {
            if !button_enabled.get_untracked() {
                notifications.set_error("Backpack already connected");
                return;
            }
            if !has_backpack.get_untracked() {
                let _ = window()
                    .open_with_url_and_target("https://chromewebstore.google.com/detail/backpack/aflkmfhebedbjioipglgcbcmnbpgliof", "_blank");
                return;
            }

            connect_wallet_in_browser().await;
        });
    };

    view! {
        <div class="flex items-start justify-start gap-4">
            <Heading>Perks</Heading>
            <button
                on:click=move |_| on_connect_button_click()
                class="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
            >

                <span class="material-symbols-outlined">wallet</span>
                {move || {
                    if has_backpack.get() {
                        if button_enabled.get() { "Connect Wallet" } else { "Wallet Connected" }
                    } else {
                        "Install Backpack"
                    }
                }}

            </button>
            <a
                rel="external"
                href="/twitter/login"
                class="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
            >
                <TwitterIcon/>

                {if perks.get().iter().any(|i| i.name == "twitter") {
                    "Twitter Connected"
                } else {
                    "Connect Twitter"
                }}

            </a>
        </div>
        <Subheading class="mt-14">Perks List</Subheading>
        <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
            <TableHead>
                <tr>
                    <TableHeader>Perk</TableHeader>
                    <TableHeader class="text-right">Multiplier</TableHeader>
                </tr>
            </TableHead>
            <tbody>

                {perks
                    .get()
                    .iter()
                    .cloned()
                    .map(|referral| {
                        view! {
                            <tr>
                                <TableCell>{referral.name.to_uppercase()}</TableCell>
                                <TableCell class="text-right">
                                    {referral.multiplier.to_string()}
                                </TableCell>
                            </tr>
                        }
                    })
                    .collect_view()}

            </tbody>
        </Table>
    }
}
