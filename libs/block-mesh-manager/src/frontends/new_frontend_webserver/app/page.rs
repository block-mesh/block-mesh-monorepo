use super::application_layout::ApplicationLayout;
use crate::frontends::new_frontend_webserver::components::{Divider, Heading, Subheading};
use leptos::*;

#[component]
pub fn Stat(
    title: &'static str,
    value: &'static str,
    icon: &'static str,
    #[prop(optional)] subtext: &'static str,
) -> impl IntoView {
    let subtext = if subtext.is_empty() {
        "".to_string()
    } else {
        format!("({subtext})")
    };

    view! {
        <div>
            <Divider class="border-blue shadow-light"/>

            <div>
                <div class="mt-6 text-lg/6 font-medium sm:text-sm/6">
                    <span>
                        {title} <small class="ml-2 text-zinc-500 stat-box-subtext">{subtext}</small>
                    </span>
                </div>
                <div class="flex justify-between items-center mt-2 text-orange">
                    <div class="text-3xl/8 font-semibold sm:text-2xl/8">
                        <span>{value}</span>
                    </div>
                    <span class="material-symbols-outlined">{icon}</span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn BandwidthCard(
    title: &'static str,
    value: &'static str,
    value_scale: &'static str,
    icon: &'static str,
) -> impl IntoView {
    view! {
        <div class="h-44 rounded-xl shadow-dark bg-bandwidth-card bg-cover p-2">
            <div class="w-full h-full rounded-lg py-[15px] px-[20px] pt-[5px] flex flex-col justify-between bg-lightDark">
                <div class="bandwidth-card-top">
                    <span class="bandwidth-card-title">{title}</span>
                </div>
                <div class="flex justify-between items-center">
                    <div class="bandwidth-card-value">
                        <span class="font-bold text-4xl">{value}</span>
                        <small>{value_scale}</small>
                    </div>
                    <div class="bandwidth-card-logo">
                        <span class="material-symbols-outlined" style="font-size: 2.3em">
                            {icon}
                        </span>
                    </div>
                </div>
            </div>
        </div>
    }
}

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
