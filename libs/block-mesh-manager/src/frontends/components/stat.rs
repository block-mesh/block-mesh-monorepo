use crate::frontends::components::divider::Divider;
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
            <Divider class="border-light-blue shadow-light"/>

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
