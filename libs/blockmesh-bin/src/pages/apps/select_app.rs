use crate::components::ore_icon::OreIcon;
use crate::page_routes::PageRoutes;
use leptos::*;

#[component]
pub fn SelectApps() -> impl IntoView {
    view! {
        <section class="p-8 lg:py-20">
            <div class="container !mx-auto text-center place-content-center grid">
                <p class="mb-2 block antialiased font-sans leading-relaxed text-blue-gray-900 !font-semibold lg:!text-lg !text-base">
                    More DePIN oppurtunities
                </p>
                <div class="flex flex-col md:flex-row gap-6 max-w-6xl mx-auto">
                    <div class="flex flex-col items-center justify-center gap-6">
                        <div class="relative flex flex-col bg-clip-border rounded-xl text-gray-700 bg-[#FAFAFA] px-10">
                            <a href=PageRoutes::OreMiner.path()>
                                <div class="p-6">
                                    <OreIcon />
                                </div>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
