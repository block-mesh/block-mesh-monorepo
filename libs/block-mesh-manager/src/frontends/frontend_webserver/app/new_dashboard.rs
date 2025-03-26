use crate::frontends::components::bandwidth_card::BandwidthCard;
use crate::frontends::components::bar_chart::BarChart;
use crate::frontends::components::download_extension::DownloadExtension;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::chrome_icon::ChromeIcon;
use crate::frontends::components::modal::Modal;
use crate::frontends::components::stat::Stat;
use crate::frontends::components::sub_heading::Subheading;
//use crate::frontends::components::tables::table::Table;
//use crate::frontends::components::tables::table_cell::TableCell;
//use crate::frontends::components::tables::table_head::TableHead;
//use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::components::edit_email::EditEmail;
use crate::frontends::context::notification_context::NotificationContext;
use block_mesh_common::constants::{BLOCK_MESH_CHROME_EXTENSION_LINK, BUTTON_CLASS};
use block_mesh_common::interfaces::server_api::{
    AuthStatusResponse, DashboardResponse, ResendConfirmEmailForm,
};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use reqwest::Client;

fn truncate_string(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();

    if chars.len() <= 6 {
        return s.to_string();
    }

    let first_three: String = chars[..3].iter().collect();
    let last_three: String = chars[chars.len() - 3..].iter().collect();

    format!("{}...{}", first_three, last_three)
}

#[component]
pub fn NewDashboard() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let async_data = use_context::<DashboardResponse>();
    let auth_status = use_context::<AuthStatusResponse>();

    let wallet_address = RwSignal::new("".to_string());
    let connected = RwSignal::new(false);
    let uptime = RwSignal::new(0.0);
    // let user_ips = RwSignal::new(vec![]);
    let verified_email = RwSignal::new(false);
    let download = RwSignal::new(0.0);
    let upload = RwSignal::new(0.0);
    let latency = RwSignal::new(0.0);
    let points = RwSignal::new(0.0);
    let tasks = RwSignal::new(0);
    let number_of_users_invited = RwSignal::new(0);
    let show_download_extension = RwSignal::new(true);
    let show_edit_email = RwSignal::new(false);
    let email = RwSignal::new("".to_string());
    if let Some(a) = auth_status {
        email.set(a.email.clone().unwrap_or_default());
    }
    let allowed_to_edit_email = Signal::derive(move || {
        email.get().ends_with("@blockmesh.xyz") && email.get().starts_with("wallet_")
    });

    if let Some(data) = async_data {
        connected.set(data.connected);
        uptime.set(data.uptime);
        // user_ips.set(data.user_ips);
        verified_email.set(data.verified_email);
        download.set(data.download);
        upload.set(data.upload);
        latency.set(data.latency);
        points.set(data.points);
        tasks.set(data.tasks);
        number_of_users_invited.set(data.number_of_users_invited);
        if let Some(w) = data.wallet_address {
            wallet_address.set(w);
        }
        if email.get_untracked().contains("wallet_")
            && email.get_untracked().contains("@blockmesh.xyz")
        {
            verified_email.set(true)
        }
        if data
            .calls_to_action
            .iter()
            .any(|i| i.name == "install_extension" && i.status)
        {
            show_download_extension.set(false)
        }
    }

    let resend_verification = create_action({
        move |_: &()| async move {
            if verified_email.get_untracked() || email.get_untracked().is_empty() {
                return;
            }

            let origin = window().origin();
            let client = Client::new();
            let response = client
                .post(format!(
                    "{}{}",
                    origin,
                    RoutesEnum::Static_Auth_ResendConfirmationEmail
                ))
                .form(&ResendConfirmEmailForm {
                    email: email.get_untracked(),
                })
                .send()
                .await;
            match response {
                Ok(r) => {
                    if r.status().as_u16() < 400 {
                        notifications.set_success("Verification email sent");
                    } else {
                        notifications.set_error("Failed to send verification email");
                    }
                }
                Err(_) => {
                    notifications.set_error("Failed to send verification email");
                }
            }
        }
    });

    view! {
        <Modal show=show_download_extension show_close_button=false>
            <DownloadExtension show=show_download_extension/>
        </Modal>
        <Modal show=show_edit_email show_close_button=true>
        <EditEmail/>
        </Modal>

        <div class="lg:flex items-start justify-start gap-4">
            <Heading>{move || format!("Dashboard v{}", env!("CARGO_PKG_VERSION"))}</Heading>
            <button
                class=BUTTON_CLASS
                on:click=move |_| {
                    if allowed_to_edit_email.get_untracked() {
                        show_edit_email.set(true);
                    }
                }
            >
                <span class="material-symbols-outlined">email</span>
                {move || email.get().to_string()}
            </button>

            <Show when=move || !wallet_address.get().is_empty()>
                <button class=BUTTON_CLASS>
                    <span class="material-symbols-outlined">wallet</span>
                    {move || truncate_string(&wallet_address.get())}
                </button>
            </Show>
            <a
                rel="external"
                target="_blank"
                href=BLOCK_MESH_CHROME_EXTENSION_LINK
                class=BUTTON_CLASS
            >
                <ChromeIcon/>
                Download ext
            </a>
            <button
                class=move || {
                    format!(
                        "-my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default {}",
                        if verified_email.get() { "text-green-600" } else { "text-red-600" },
                    )
                }

                on:click=move |_| { resend_verification.dispatch(()) }
            >
                <span class="material-symbols-outlined">
                    {move || if verified_email.get() { "check" } else { "close" }}
                </span>
                {move || {
                    if verified_email.get() {
                        "Email Verified"
                    } else {
                        "Click to resend verification email"
                    }
                }}

            </button>
        </div>

        <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-5">
            <Stat
                title="Connection Status"
                value=move || {
                    (if connected.get() { "Connected" } else { "Disconnected" }).to_string()
                }

                icon="wifi"
            />
            // subtext="seconds"
            <Stat title="Uptime" value=move || format!("{:.1}", uptime.get()) icon="trending_up"/>
            // subtext="seconds"
            <Stat
                title="# Invites"
                value=move || format!("{:.1}", number_of_users_invited.get())
                icon="notification_multiple"
            />
            <Stat title="# Tasks" value=move || format!("{:.1}", tasks.get()) icon="task_alt"/>
            <Stat title="Points" value=move || format!("{:.1}", points.get()) icon="my_location"/>
        </div>
        <br/>
        <br/>
        <Subheading>Bandwidth Statistics</Subheading>
        <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-3">
            <BandwidthCard
                title="Download Speed"
                value=move || format!("{:.1}", download.get())
                icon="download"
                value_scale="Mbps"
            />
            <BandwidthCard
                title="Upload Speed"
                value=move || format!("{:.1}", upload.get())
                icon="upload"
                value_scale="Mbps"
            />
            <BandwidthCard
                title="Latency"
                value=move || format!("{:.1}", latency.get())
                icon="network_check"
                value_scale="ms"
            />
        </div>
        // <Subheading>Networks</Subheading>
        // <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
        // <TableHead>
        // <tr>
        // <TableHeader>IP</TableHeader>
        // <TableHeader>Country</TableHeader>
        // </tr>
        // </TableHead>
        // <tbody>
        // <Suspense>
        // {user_ips
        // .get()
        // .into_iter()
        // .map(|ip_info| {
        // view! {
        // <tr>
        // <TableCell>{ip_info.ip.clone()}</TableCell>
        // <TableCell>{ip_info.country.clone()}</TableCell>
        // </tr>
        // }
        // })
        // .collect_view()}
        // </Suspense>
        // </tbody>
        // </Table>
        <Subheading>Daily points earnings</Subheading>
        <BarChart/>
    }
}
