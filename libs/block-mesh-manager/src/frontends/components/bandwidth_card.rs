use leptos::*;

#[component]
pub fn BandwidthCard<F>(
    title: &'static str,
    value: F,
    value_scale: &'static str,
    icon: &'static str,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    view! {
        <div class="text-off-white h-44 rounded-xl shadow-dark bg-cover p-2 border border-white">
            <div class="w-full h-full rounded-lg py-[15px] px-[20px] pt-[5px] flex flex-col justify-between">
                <div class="bandwidth-card-top">
                    <span class="font-bebas-neue">{title}</span>
                </div>
                <div class="flex justify-between items-center">
                    <div class="font-open-sans">
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
