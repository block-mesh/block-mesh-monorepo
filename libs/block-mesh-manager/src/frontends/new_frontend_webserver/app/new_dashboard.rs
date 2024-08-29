use super::application_layout::ApplicationLayout;
use crate::frontends::components::bandwidth_card::BandwidthCard;
use crate::frontends::components::bar_chart::BarChart;
use crate::frontends::components::download_extension::DownloadExtension;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::icons::checkmark_icon::CheckMarkIcon;
use crate::frontends::components::icons::chrome_icon::ChromeIcon;
use crate::frontends::components::icons::xmark_icon::XMarkIcon;
use crate::frontends::components::modal::Modal;
use crate::frontends::components::stat::Stat;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::webapp_context::WebAppContext;
use block_mesh_common::constants::BLOCK_MESH_CHROME_EXTENSION_LINK;
use block_mesh_common::interfaces::server_api::ResendConfirmEmailForm;
use block_mesh_common::routes_enum::RoutesEnum;
use chrono::Utc;
use leptos::*;
use reqwest::Client;

#[component]
pub fn NewDashboard() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let auth_data = WebAppContext::is_logged_in();
    let async_data = WebAppContext::get_dashboard_data();
    let show_download_extension = create_rw_signal(true);

    create_effect(move |_| {
        if let Some(Some(response)) = async_data.get() {
            if response
                .calls_to_action
                .iter()
                .any(|i| i.name == "install_extension" && i.status)
            {
                show_download_extension.set(false);
            }
        }
    });

    let verified_email = Signal::derive(move || {
        if let Some(Some(data)) = async_data.get() {
            data.verified_email
        } else {
            false
        }
    });
    let connected = Signal::derive(move || {
        if let Some(Some(data)) = async_data.get() {
            data.connected
        } else {
            false
        }
    });
    let email = Signal::derive(move || {
        if let Some(Some(data)) = auth_data.get() {
            data.email
        } else {
            None
        }
    });

    let resend_verification = create_action(move |_: &()| async move {
        if verified_email.get_untracked() || email.get_untracked().is_none() {
            return;
        }
        let origin = window().origin();
        let client = Client::new();
        let response = client
            .post(format!(
                "{}{}",
                origin,
                RoutesEnum::Static_UnAuth_ResendConfirmationEmail
            ))
            .form(&ResendConfirmEmailForm {
                email: email.get_untracked().unwrap_or_default(),
            })
            .send()
            .await;
        match response {
            Ok(_) => {
                notifications.set_success("Verification email sent");
            }
            Err(_) => {
                notifications.set_error("Failed to send verification email");
            }
        }
    });

    let connected_status = Signal::derive(move || {
        if connected.get() {
            "Connected"
        } else {
            "Disconnected"
        }
        .to_string()
    });

    let points = Signal::derive(move || {
        let p = if let Some(Some(i)) = async_data.get() {
            i.points
        } else {
            0.0
        };
        format!("{:.1}", p)
    });
    let tasks = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.tasks
        } else {
            0
        };
        format!("{:.1}", v)
    });
    let uptime = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.uptime
        } else {
            0.0
        };
        format!("{:.1}", v)
    });
    let invites = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.number_of_users_invited
        } else {
            0
        };
        format!("{:.1}", v)
    });
    let download = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.download
        } else {
            0.0
        };
        format!("{:.1}", v)
    });
    let upload = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.upload
        } else {
            0.0
        };
        format!("{:.1}", v)
    });
    let latency = Signal::derive(move || {
        let v = if let Some(Some(i)) = async_data.get() {
            i.latency
        } else {
            0.0
        };
        format!("{:.1}", v)
    });

    let user_ips = Signal::derive(move || {
        if let Some(Some(data)) = async_data.get() {
            data.user_ips
        } else {
            Vec::new()
        }
    });

    view! {
        <ApplicationLayout>
            <Modal show=show_download_extension show_close_button=false>
                <DownloadExtension show=show_download_extension/>
            </Modal>
            <Suspense fallback=move || view! {}>
                <div class="flex items-start justify-start gap-4">
                    <Heading>Dashboard</Heading>
                    <a
                        rel="external"
                        target="_blank"
                        href=BLOCK_MESH_CHROME_EXTENSION_LINK
                        class="text-magenta-2 -my-0.5 cursor-pointer relative isolate inline-flex items-center justify-center gap-x-2 rounded-lg border text-base/6 font-semibold px-[calc(theme(spacing[3.5])-1px)] py-[calc(theme(spacing[2.5])-1px)] sm:px-[calc(theme(spacing.3)-1px)] sm:py-[calc(theme(spacing[1.5])-1px)] sm:text-sm/6 focus:outline-none data-[focus]:outline data-[focus]:outline-2 data-[focus]:outline-offset-2 data-[focus]:outline-blue-500 data-[disabled]:opacity-50 [&>[data-slot=icon]]:-mx-0.5 [&>[data-slot=icon]]:my-0.5 [&>[data-slot=icon]]:size-5 [&>[data-slot=icon]]:shrink-0 [&>[data-slot=icon]]:text-[--btn-icon] [&>[data-slot=icon]]:sm:my-1 [&>[data-slot=icon]]:sm:size-4 forced-colors:[--btn-icon:ButtonText] forced-colors:data-[hover]:[--btn-icon:ButtonText] border-transparent bg-[--btn-border] bg-[--btn-bg] before:absolute before:inset-0 before:-z-10 before:rounded-[calc(theme(borderRadius.lg)-1px)] before:bg-[--btn-bg] before:shadow before:hidden border-white/5 after:absolute after:inset-0 after:-z-10 after:rounded-[calc(theme(borderRadius.lg)-1px)] after:shadow-[shadow:inset_0_1px_theme(colors.white/15%)] after:data-[active]:bg-[--btn-hover-overlay] after:data-[hover]:bg-[--btn-hover-overlay] after:-inset-px after:rounded-lg before:data-[disabled]:shadow-none after:data-[disabled]:shadow-none [--btn-bg:theme(colors.zinc.900)] [--btn-border:theme(colors.zinc.950/90%)] [--btn-hover-overlay:theme(colors.white/10%)] [--btn-bg:theme(colors.zinc.600)] [--btn-hover-overlay:theme(colors.white/5%)] [--btn-icon:theme(colors.zinc.400)] data-[active]:[--btn-icon:theme(colors.zinc.300)] data-[hover]:[--btn-icon:theme(colors.zinc.300)] cursor-default"
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
                        value=move || connected_status.get()
                        icon="wifi"
                    />
                    // subtext="seconds"
                    <Stat title="Uptime" value=move || uptime.get() icon="trending_up"/>
                    // subtext="seconds"
                    <Stat
                        title="# Invites"
                        value=move || invites.get()
                        icon="notification_multiple"
                    />
                    <Stat title="# Tasks" value=move || tasks.get() icon="task_alt"/>
                    <Stat title="Points" value=move || points.get() icon="my_location"/>
                </div>
                <br/>
                <br/>
                <Subheading>Bandwidth Statistics</Subheading>
                <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-3">
                    <BandwidthCard
                        title="Download Speed"
                        value=move || download.get()
                        icon="download"
                        value_scale="Mbps"
                    />
                    <BandwidthCard
                        title="Upload Speed"
                        value=move || upload.get()
                        icon="upload"
                        value_scale="Mbps"
                    />
                    <BandwidthCard
                        title="Latency"
                        value=move || latency.get()
                        icon="network_check"
                        value_scale="ms"
                    />
                </div>
                <Subheading>Networks</Subheading>
                <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
                    <TableHead>
                        <tr>
                            <TableHeader>IP</TableHeader>
                            <TableHeader>Country</TableHeader>
                            <TableHeader>Active</TableHeader>
                        </tr>
                    </TableHead>
                    <tbody>
                        <Suspense>
                            {user_ips
                                .get()
                                .into_iter()
                                .map(|ip_info| {
                                    view! {
                                        <tr>
                                            <TableCell>{ip_info.ip.clone()}</TableCell>
                                            <TableCell>{ip_info.country.clone()}</TableCell>
                                            <TableCell>
                                                {
                                                    let now = Utc::now();
                                                    let diff = now - ip_info.updated_at;
                                                    if diff.num_seconds() > 3600 {
                                                        view! { <XMarkIcon/> }
                                                    } else {
                                                        view! { <CheckMarkIcon/> }
                                                    }
                                                }
                                            </TableCell>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </Suspense>
                    </tbody>
                </Table>
                <Subheading>Daily points earnings</Subheading>
                <BarChart/>
            </Suspense>
        </ApplicationLayout>
    }
}
