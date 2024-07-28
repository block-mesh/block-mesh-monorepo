use leptos::*;

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
