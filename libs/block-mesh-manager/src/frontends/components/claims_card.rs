use leptos::*;

#[component]
pub fn ClaimsCard(
    title: &'static str,
    title_link: &'static str,
    value: &'static str,
    value_scale: &'static str,
    dexscreener_link: &'static str,
    cookie_fun_link: &'static str,
    disabled: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let (view, _) = create_signal(move || children().into_view());

    view! {
        <div class="text-off-white h-44 rounded-xl shadow-dark hover:bg-orange bg-cover border-2 border-cyan">
            <div class="w-full h-full rounded-lg py-[15px] px-[20px] pt-[5px] flex flex-col justify-between bg-lightDark">
                <div class="bandwidth-card-top">
                    <span class="font-bebas-neue">
                        <Show
                            when=move || { !disabled }
                            fallback=move || {
                                view! { <div>{title}</div> }
                            }
                        >

                            <a href=title_link target="_blank">
                                {title}
                            </a>
                        </Show>

                    </span>
                </div>
                <div class="flex justify-between items-center">
                    <div class="font-open-sans">
                        <Show
                            when=move || { !disabled }
                            fallback=move || {
                                view! {
                                    <span class="font-bold text-4xl">{value}</span>
                                    <small>{value_scale}</small>
                                }
                            }
                        >

                            <a href=dexscreener_link target="_blank">
                                <span class="font-bold text-4xl">{value}</span>
                                <small>{value_scale}</small>
                            </a>
                        </Show>
                    </div>
                    <div class="bandwidth-card-logo">
                        <Show
                            when=move || { !disabled }
                            fallback=move || {
                                view! { {view.get()} }
                            }
                        >
                            <a href=cookie_fun_link target="_blank">
                                {view.get()}
                            </a>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}
