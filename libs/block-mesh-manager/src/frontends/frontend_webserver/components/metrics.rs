use crate::frontends::frontend_webserver::components::icons::chrome_icon::ChromeIcon;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use block_mesh_common::constants::BLOCK_MESH_CHROME_EXTENSION_LINK;
use leptos::*;

#[component]
pub fn MetricsComponent() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    view! {
        <div class="m-2">
            <div class="border-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
                <img
                    alt="back"
                    loading="lazy"
                    width="808"
                    height="433"
                    decoding="async"
                    data-nimg="1"
                    class="absolute top-0 right-0 h-full w-[70%] max-w-[800px] object-cover object-left"
                    src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/35b28e8e-2665-4c8d-a5cf-d7354fce4a00/public"
                    style="color: transparent;"
                />
                <div class="relative z-[1]">
                    <div class="flex items-center text-[#808080] leading-[153%] max-md:text-xs max-md:ml-1.5 text-green-300">
                        <svg
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            width="16"
                            height="16"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M11.48 3.499a.562.562 0 0 1 1.04 0l2.125 5.111a.563.563 0 0 0 .475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 0 0-.182.557l1.285 5.385a.562.562 0 0 1-.84.61l-4.725-2.885a.562.562 0 0 0-.586 0L6.982 20.54a.562.562 0 0 1-.84-.61l1.285-5.386a.562.562 0 0 0-.182-.557l-4.204-3.602a.562.562 0 0 1 .321-.988l5.518-.442a.563.563 0 0 0 .475-.345L11.48 3.5Z"
                            ></path>
                        </svg>

                        Overall Points
                    </div>
                    <div class="text-green-300 mt-[7px] md:mt-2.5 text-[30px] md:text-[40px] font-semibold leading-[116%] max-md:ml-1.5">
                        {move || {
                            let p = match async_data.get() {
                                Some(Some(response)) => response.points,
                                _ => 0f64,
                            };
                            format!("{:.3}", p)
                        }}

                    </div>
                    <p class="text-sm font-medium leading-6 text-gray-400">Chrome Extension</p>
                    <p class="mt-2 flex items-baseline gap-x-2 text-white">
                        <a
                            href=BLOCK_MESH_CHROME_EXTENSION_LINK
                            target="_blank"
                            class="inline-flex items-center gap-x-1.5 rounded-md border px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-gray-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                        >
                            Download
                            <ChromeIcon/>
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}
