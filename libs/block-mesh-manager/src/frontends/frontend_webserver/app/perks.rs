use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::info_icon::InfoIcon;
use crate::frontends::components::icons::person_icon::PersonIcon;
use crate::frontends::components::icons::twitter_icon::TwitterIcon;
use crate::frontends::components::modal::Modal;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::components::wallet_selector::WalletSelector;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::utils::auth::connect_wallet_in_browser;
use block_mesh_common::constants::{
    BLOCKMESH_FOUNDER_TWITTER_USER_ID, BLOCKMESH_TWITTER_USER_ID, BUTTON_CLASS,
    XENO_TWITTER_USER_ID,
};
use block_mesh_common::interfaces::server_api::{AuthStatusResponse, DashboardResponse};
use leptos::*;

#[component]
pub fn Perks() -> impl IntoView {
    let async_data = use_context::<DashboardResponse>();
    let auth_status = use_context::<AuthStatusResponse>();

    let notifications = expect_context::<NotificationContext>();
    let show_wallet_modal = RwSignal::new(false);
    let wallet_name = RwSignal::new("".to_string());
    let perks = RwSignal::new(vec![]);
    let button_enabled = RwSignal::new(true);
    let wallet_address = RwSignal::new("".to_string());
    let enable_proof_of_humanity = RwSignal::new(false);

    if let Some(a) = auth_status {
        enable_proof_of_humanity.set(a.enable_proof_of_humanity);
        let wallet = a.wallet_address.unwrap_or_default();
        let (first_4, last_4) = (
            &wallet[..wallet.len().min(4)],
            &wallet[wallet.len().saturating_sub(4)..],
        );
        wallet_address.set(format!("{}...{}", first_4, last_4));
    }

    if let Some(data) = async_data {
        perks.set(data.perks);
        button_enabled.set(data.wallet_address.is_none());
    }
    let on_connect_button_click = move || {
        // if button_enabled.get() {
        show_wallet_modal.set(true);
        // }
    };

    let connect_action = create_action(move |wallet: &String| {
        let w = wallet.clone();
        async move {
            // if !button_enabled.get_untracked() {
            //     notifications.set_error("Wallet already connected");
            //     return;
            // }
            if connect_wallet_in_browser(w).await {
                // button_enabled.set(false)
                notifications.set_success("Wallet connected");
            }
        }
    });

    view! {
        <Modal show=show_wallet_modal show_close_button=true>
            <WalletSelector show=show_wallet_modal wallet_name=wallet_name connect=connect_action/>
        </Modal>
        <div class="lg:flex items-start justify-start gap-4">
            <Heading>Perks</Heading>
            <Show when=move || enable_proof_of_humanity.get() fallback=|| view! {}>
                <a
                    rel="external"
                    href="/proof_of_humanity"
                    class=BUTTON_CLASS
                >
                    <PersonIcon/>
                    "Proof of Humanity"
                </a>
                <a
                    rel="external"
                    target="_blank"
                    href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/PROOF_OF_HUMANITY.md"
                >
                    <InfoIcon/>
                </a>
            </Show>
            <button
                on:click=move |_| on_connect_button_click()
                class=BUTTON_CLASS
            >
                <span class="material-symbols-outlined">wallet</span>
                {move || {
                    if button_enabled.get() {
                        "Connect Wallet".to_string()
                    } else {
                        format!("Wallet Connected:  {}", wallet_address.get())
                    }
                }}
            </button>
               <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/CONNECT_WALLET.md"
            >
                <InfoIcon/>
            </a>
            <a
                rel="external"
                href=format!("/twitter/login?target={}", BLOCKMESH_TWITTER_USER_ID)
                class=BUTTON_CLASS
            >
                <TwitterIcon/>

                {move || {
                    if perks.get().iter().any(|i| i.name == "twitter") {
                        "Twitter Connected"
                    } else {
                        "Connect Twitter"
                    }
                }}
            </a>
            <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            >
                <InfoIcon/>
            </a>
            <a
                rel="external"
                href=format!("/twitter/login?target={}", BLOCKMESH_FOUNDER_TWITTER_USER_ID)
                class=BUTTON_CLASS
            >
                <TwitterIcon/>
                {move || {
                    if perks.get().iter().any(|i| i.name == "founder_twitter") {
                        "Foundered Followed"
                    } else {
                        "Follow Founder"
                    }
                }}
            </a>
            <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            >
                <InfoIcon/>
            </a>
                <a
                rel="external"
                href=format!("/twitter/login?target={}", XENO_TWITTER_USER_ID)
                class=BUTTON_CLASS
            >
                <TwitterIcon/>

                {move || {
                    if perks.get().iter().any(|i| i.name == "xeno_twitter") {
                        "Xenopus Followed"
                    } else {
                        "Follow Xenopus"
                    }
                }}

            </a>
            <a
                rel="external"
                target="_blank"
                href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/TWITTER_PERK.md"
            >
                <InfoIcon/>
            </a>
        </div>
        <Subheading class="mt-14">Perks List</Subheading>
        <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
            <TableHead>
                <tr>
                    <TableHeader>Perk</TableHeader>
                    <TableHeader>One Time Bonus</TableHeader>
                    <TableHeader class="text-right">Multiplier</TableHeader>
                </tr>
            </TableHead>
            <tbody>
                {move || {
                    perks
                        .get()
                        .iter()
                        .cloned()
                        .map(|referral| {
                            view! {
                                <tr>
                                    <TableCell>{referral.name.to_uppercase()}</TableCell>
                                    <TableCell>{referral.one_time_bonus.to_string()}</TableCell>
                                    <TableCell class="text-right">
                                        {referral.multiplier.to_string()}
                                    </TableCell>
                                </tr>
                            }
                        })
                        .collect_view()
                }}

            </tbody>
        </Table>
    }
}
