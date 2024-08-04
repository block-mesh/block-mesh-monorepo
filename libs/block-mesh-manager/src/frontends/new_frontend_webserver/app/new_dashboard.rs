use super::application_layout::ApplicationLayout;
use crate::frontends::components::bandwidth_card::BandwidthCard;
use crate::frontends::components::bar_chart::BarChart;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::stat::Stat;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::context::webapp_context::WebAppContext;
use leptos::*;

#[component]
pub fn NewDashboard() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let connected = Signal::derive(move || {
        if let Some(data) = async_data.get() {
            data.is_some_and(|d| d.connected)
        } else {
            false
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

    view! {
        <ApplicationLayout>
            <Suspense fallback=move || view! {}>
                <Heading>Dashboard</Heading>
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
                    <Stat
                        title="# Tasks"
                        value=move || tasks.get()
                        icon="task_alt"
                    />
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
                <Subheading>Daily points earnings</Subheading>
                <BarChart/>
            </Suspense>
        </ApplicationLayout>
    }
}
