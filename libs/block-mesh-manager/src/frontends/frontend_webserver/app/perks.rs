use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::info_icon::InfoIcon;
use crate::frontends::components::icons::intract_icon::IntractIcon;
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
use crate::frontends::frontend_webserver::app::perks_data::{get_perks_data, show_perk};
use crate::frontends::utils::auth::connect_wallet_in_browser;
use crate::frontends::utils::perk_util::sync_perk;
use block_mesh_common::constants::{
    BLOCKMESH_FOUNDER_TWITTER_USER_ID, BLOCKMESH_TWITTER_USER_ID, BUTTON_CLASS, WOOTZ_APP_USER_ID,
    XENO_TWITTER_USER_ID,
};
use block_mesh_common::interfaces::server_api::{AuthStatusResponse, DashboardResponse};
use leptos::*;

#[component]
pub fn Perks() -> impl IntoView {
    let perks_data: RwSignal<Vec<(String, String)>> =
        RwSignal::new(get_perks_data().into_iter().collect());
    let async_data = use_context::<DashboardResponse>();
    let auth_status = use_context::<AuthStatusResponse>();
    let notifications = expect_context::<NotificationContext>();
    let show_wallet_modal = RwSignal::new(false);
    let wallet_name = RwSignal::new("".to_string());
    let perks = RwSignal::new(vec![]);
    let button_enabled = RwSignal::new(true);
    let wallet_address = RwSignal::new("".to_string());
    let enable_proof_of_humanity = RwSignal::new(false);
    let email = RwSignal::new("".to_string());

    if let Some(a) = auth_status {
        enable_proof_of_humanity.set(a.enable_proof_of_humanity);
        let wallet = a.wallet_address.unwrap_or_default();
        let (first_4, last_4) = (
            &wallet[..wallet.len().min(4)],
            &wallet[wallet.len().saturating_sub(4)..],
        );
        wallet_address.set(format!("{}...{}", first_4, last_4));
        email.set(a.email.clone().unwrap_or_default());
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

    let sync_intract = create_action(move |_: &()| async move {
        if email.get_untracked().is_empty() {
            return;
        }
        match sync_perk().await {
            Ok(response) => {
                if response.error {
                    notifications.set_error(format!(
                        "Error from Intract: {}",
                        response.message.unwrap_or_default()
                    ));
                } else if response.cached {
                    notifications.set_success("Please try again in 10min");
                } else {
                    notifications.set_success("Intract synced");
                }
            }
            Err(_) => {
                notifications.set_error(format!(
                    "Failed to sync Intract , please ensure your Intract email is {}",
                    email.get_untracked().to_lowercase()
                ));
            }
        }
    });

    let connect_action = create_action(move |wallet: &String| {
        let w = wallet.clone();
        async move {
            match connect_wallet_in_browser(w).await {
                Ok(_) => {
                    notifications.set_success("Wallet connected");
                }
                Err(e) => {
                    notifications.set_error(e.to_string());
                }
            }
        }
    });

    view! {
        <Modal show=show_wallet_modal show_close_button=true>
            <WalletSelector show=show_wallet_modal wallet_name=wallet_name connect=connect_action/>
        </Modal>
        <div class="lg:flex items-start justify-start gap-4">
            <Heading>Perks</Heading>
            <Show when=move || { show_perk(&perks.get(), "intract") }>
                <button on:click=move |_| sync_intract.dispatch(()) class=BUTTON_CLASS>
                    <IntractIcon/>
                    "Intract"
                </button>
                <a
                    rel="external"
                    target="_blank"
                    href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/INTRACT_PERK.md"
                >
                    <InfoIcon/>
                </a>
            </Show>
            <Show when=move || { show_perk(&perks.get(), "proof_of_humanity") }>
                <Show when=move || enable_proof_of_humanity.get() fallback=|| view! {}>
                    <a rel="external" href="/proof_of_humanity" class=BUTTON_CLASS>
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
            </Show>
            <Show when=move || { show_perk(&perks.get(), "wallet") }>
                <button on:click=move |_| on_connect_button_click() class=BUTTON_CLASS>
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
            </Show>
            <Show when=move || { show_perk(&perks.get(), "twitter") }>
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
            </Show>
            <Show when=move || { show_perk(&perks.get(), "founder_twitter") }>
                <a
                    rel="external"
                    href=format!("/twitter/login?target={}", BLOCKMESH_FOUNDER_TWITTER_USER_ID)
                    class=BUTTON_CLASS
                >
                    <TwitterIcon/>
                    {move || {
                        if perks.get().iter().any(|i| i.name == "founder_twitter") {
                            "Founder Followed"
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
            </Show>
            <Show when=move || { show_perk(&perks.get(), "xeno_twitter") }>
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
            </Show>
            <Show when=move || { show_perk(&perks.get(), "wootz_twitter") }>
                <a
                    rel="external"
                    href=format!("/twitter/login?target={}", WOOTZ_APP_USER_ID)
                    class=BUTTON_CLASS
                >
                    <TwitterIcon/>

                    {move || {
                        if perks.get().iter().any(|i| i.name == "wootz_twitter") {
                            "WootzApp Followed"
                        } else {
                            "Follow WootzApp"
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
            </Show>
        </div>
        <Subheading class="mt-14">Perks List</Subheading>
        <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
            <TableHead>
                <tr>
                    <TableHeader>Perk</TableHeader>
                    <TableHeader>Info</TableHeader>
                </tr>
            </TableHead>
            <tbody>
                {move || {
                    perks_data
                        .get()
                        .iter()
                        .cloned()
                        .map(|(key, val)| {
                            view! {
                                <tr>
                                    <TableCell>{key.to_uppercase()}</TableCell>
                                    <TableCell>
                                        <a rel="external" target="_blank" href=val>
                                            <InfoIcon/>
                                        </a>
                                    </TableCell>
                                </tr>
                            }
                        })
                        .collect_view()
                }}

            </tbody>
        </Table>

        <Subheading class="mt-14">Completed Perks List</Subheading>
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
