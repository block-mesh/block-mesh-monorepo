use super::application_layout::ApplicationLayout;
use crate::frontends::components::bandwidth_card::BandwidthCard;
use crate::frontends::components::heading::Heading;
use crate::frontends::components::stat::Stat;
use crate::frontends::components::sub_heading::Subheading;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ApplicationLayout>
            <Heading>Dashboard</Heading>
            <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-4">
                <Stat title="Uptime" value="3,584" icon="trending_up" subtext="seconds"/>
                <Stat title="Number of invites" value="455" icon="notification_multiple"/>
                <Stat title="Number of tasks performed" value="5,888" icon="task_alt"/>
                <Stat title="Points Accumulated" value="8,524.45" icon="my_location"/>
            </div>

            <br/>
            <br/>

            <Subheading>Bandwidth Statistics</Subheading>
            <div class="mt-10 grid gap-8 sm:grid-cols-2 xl:grid-cols-3">
                <BandwidthCard
                    title="Download Speed"
                    value="58"
                    icon="download"
                    value_scale="Mbps"
                />
                <BandwidthCard title="Upload Speed" value="21" icon="upload" value_scale="Mbps"/>
                <BandwidthCard title="Latency" value="125" icon="network_check" value_scale="ms"/>
            </div>
        </ApplicationLayout>
    }
}
