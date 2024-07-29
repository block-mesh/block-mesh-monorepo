use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use leptos::*;

#[component]
pub fn NetworkStatusComponent() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let connected = Signal::derive(move || {
        if let Some(data) = async_data.get() {
            data.is_some_and(|d| d.connected)
        } else {
            false
        }
    });

    let status_color = Signal::derive(move || {
        if connected.get() {
            "green-300"
        } else {
            "red-600"
        }
    });

    view! {
        <div class="border-off-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <img
                alt="back"
                loading="lazy"
                width="808"
                height="433"
                decoding="async"
                data-nimg="1"
                class="opacity-30 absolute top-0 right-0 h-full w-[70%] max-w-[800px] object-cover object-left"
                src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/b589b8b1-ae33-488c-e7d6-0259a1cd0d00/public"
                style="color: transparent;"
            />
            <div class="relative z-[1]">
                <div class="flex items-center text-[#808080] leading-[153%] max-md:text-xs max-md:ml-1.5 text-off-white">
                    Network Status
                </div>
                <div class="mt-[7px] md:mt-2.5 text-[30px] md:text-[40px] font-semibold leading-[116%] max-md:ml-1.5">
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class=move || format!("text-{}", status_color.get())
                        width="36"
                        height="36"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            d="M8.288 15.038a5.25 5.25 0 0 1 7.424 0M5.106 11.856c3.807-3.808 9.98-3.808 13.788 0M1.924 8.674c5.565-5.565 14.587-5.565 20.152 0M12.53 18.22l-.53.53-.53-.53a.75.75 0 0 1 1.06 0Z"
                        ></path>
                    </svg>
                </div>
                <p class="text-sm font-medium leading-6 text-off-white">
                    {move || {
                        if connected.get() {
                            view! {
                                <div class=move || {
                                    format!("text-{}", status_color.get())
                                }>Your device is connected</div>
                            }
                        } else {
                            view! {
                                <div class=move || {
                                    format!("text-{}", status_color.get())
                                }>Your device is offline</div>
                            }
                        }
                    }}

                </p>
            </div>
        </div>
    }
}
