use leptos::*;
use leptos_router::use_location;
use tailwind_fuse::*;

#[component]
pub fn OnlineChip(#[prop(into)] is_online: MaybeSignal<bool>) -> impl IntoView {
    let span_class = move || {
        tw_join!(
            "h-2 w-2 mr-2",
            "rounded-full",
            if is_online.get() {
                "bg-blue shadow-blue"
            } else {
                "bg-darkOrange shadow-darkOrange"
            }
        )
    };

    view! {
        <div class="rounded-lg px-2 flex items-center text-gray-400 ml-auto bg-light">
            <span class=span_class></span>
            <span>{move || if is_online.get() { "Online" } else { "Offline" }}</span>
        </div>
    }
}

#[component]
pub fn ApplicationLayout(children: Children) -> impl IntoView {
    let pathname = use_location().pathname;

    view! {

    }
}
