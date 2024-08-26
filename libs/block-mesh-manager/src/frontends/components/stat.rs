use crate::frontends::components::divider::Divider;
use leptos::*;

#[component]
pub fn Stat<F>(
    title: &'static str,
    value: F,
    icon: &'static str,
    #[prop(optional)] subtext: &'static str,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    let subtext = if subtext.is_empty() {
        "".to_string()
    } else {
        format!("({subtext})")
    };

    view! {
        <div>
            <Divider class="border-cyan"/>

            <div>
                <div class="font-bebas-neue text-off-white mt-6 text-lg/6 font-medium sm:text-sm/6">
                    <span class="font-open-sans">
                        {title}
                        <small class="ml-2 text-off-white stat-box-subtext">{subtext}</small>
                    </span>
                </div>
                <div class="flex justify-between items-center mt-2 text-orange">
                    <div class="font-open-sans text-3xl/8 font-semibold sm:text-2xl/8">
                        <span>{value}</span>
                    </div>
                    <span class="material-symbols-outlined">{icon}</span>
                </div>
            </div>
        </div>
    }
}
