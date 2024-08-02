use crate::frontends::context::webapp_context::WebAppContext;
use leptos::*;

#[component]
pub fn PerksComponent() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let data = Signal::derive(move || {
        if let Some(Some(i)) = async_data.get() {
            i.perks
        } else {
            vec![]
        }
    });

    view! {
        <div class="border-off-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <div class="relative z-[1]">
                <div class="grid grid-cols-2 flex items-center text-[#808080] leading-[153%] max-md:text-xs max-md:ml-1.5 text-magenta">
                    <div>Perk</div>
                    <div>Multiplier</div>
                </div>
            </div>
            <For
                each=move || data.get()
                key=|perk| perk.id
                children=move |perk| {
                    view! {
                        <div class="grid grid-cols-2 rounded">
                            <div class="text-cyan">{perk.name.to_uppercase()}</div>
                            <div class="text-cyan">{perk.multiplier}</div>
                        </div>
                    }
                }
            />

        </div>
    }
}
