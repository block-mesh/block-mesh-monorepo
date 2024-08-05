use crate::frontends::components::heading::Heading;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use crate::frontends::utils::auth::connect_wallet;
use crate::frontends::utils::connectors::{pubkey, sign_message};
use block_mesh_common::interfaces::server_api::ConnectWalletRequest;
use js_sys::Uint8Array;
use leptos::*;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[component]
pub fn Perks() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let notifications = expect_context::<NotificationContext>();
    let logged_in = WebAppContext::is_logged_in();
    let wallet = Signal::derive(move || {
        if let Some(Some(l)) = logged_in.get() {
            l.wallet_address
        } else {
            None
        }
    });
    let button_enable = Signal::derive(move || wallet.get().is_none());
    let perks = Signal::derive(move || {
        if let Some(Some(i)) = async_data.get() {
            i.perks
        } else {
            vec![]
        }
    });
    let click_button = move || {
        spawn_local(async move {
            if !button_enable.get() {
                notifications.set_error("Backpack already connected");
                return;
            }
            if !window().has_own_property(&JsValue::from_str("backpack")) {
                notifications.set_error("Backpack wallet not found");
            }
            let msg = Uuid::new_v4().to_string();
            let key = pubkey().await;
            let sign = sign_message(&msg).await;
            let uint8_array = Uint8Array::new(&sign);
            let mut vec = vec![0; uint8_array.length() as usize];
            uint8_array.copy_to(&mut vec[..]);
            let origin = window().origin();
            match connect_wallet(
                origin,
                ConnectWalletRequest {
                    pubkey: key.as_string().unwrap(),
                    message: msg.to_string(),
                    signature: vec,
                },
            )
            .await
            {
                Ok(_) => {
                    logged_in.refetch();
                    async_data.refetch();
                    notifications.set_success("Connected successfully");
                }
                Err(_) => notifications.set_error("Failed to connect"),
            }
        })
    };

    view! {
        <ApplicationLayout>
            <div class="flex items-start justify-start gap-4">
                <Heading>Perks</Heading>
                <button
                    on:click=move |_| click_button()
                    class="-my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none text-white [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] :text-white [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
                >

                    <span class="material-symbols-outlined">wallet</span>
                    {move || {
                        if button_enable.get() { "Connect Wallet" } else { "Wallet Connected" }
                    }}

                </button>
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
                    <Suspense>
                        {perks
                            .get()
                            .into_iter()
                            .map(|referral| {
                                view! {
                                    <tr>
                                        <TableCell>{referral.name.clone().to_uppercase()}</TableCell>
                                        <TableCell class="text-right">
                                            {referral.multiplier.to_string()}
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
