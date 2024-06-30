use crate::leptos_state::LeptosTauriAppState;
use crate::page_routes::PageRoutes;
use leptos::*;
use leptos_router::use_location;

#[component]
pub fn Sidebar() -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let location = use_location();
    let current_path = Signal::derive(move || location.pathname.get());
    let active = move |path: &str| {
        if path == current_path.get() {
            "bg-gray-800 text-white group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
        } else {
            "text-gray-400 hover:text-white hover:bg-gray-800 group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold"
        }
    };

    view! {
        <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col border-r-2  border-r-white">
            <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-4">
                <div class="flex h-16 shrink-0 items-center">
                    <img
                        class="h-8 w-auto"
                        src="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
                        alt="BlockMesh"
                    />
                </div>
                <nav class="flex flex-1 flex-col">
                    <ul role="list" class="flex flex-1 flex-col gap-y-7">
                        <li>
                            <ul role="list" class="-mx-2 space-y-1">
                                <li>
                                    <a
                                        href=PageRoutes::Home.path()
                                        class=move || active(PageRoutes::Home.path())
                                    >
                                        BlockMesh
                                    </a>
                                </li>
                                // <li>
                                // <a
                                // href=PageRoutes::Dashboard.path()
                                // class=move || active(PageRoutes::Dashboard.path())
                                // >
                                // Dashboard
                                // </a>
                                // </li>
                                // <li>
                                // <a
                                // href=PageRoutes::Settings.path()
                                // class=move || active(PageRoutes::Settings.path())
                                // >
                                // Settings
                                // </a>
                                // </li>
                                <Show
                                    when=move || { state.logged_in.get() }
                                    fallback=move || {
                                        view! {}
                                    }
                                >

                                    <li>
                                        <a
                                            href=PageRoutes::Apps.path()
                                            class=move || active(PageRoutes::Apps.path())
                                        >
                                            Apps
                                        </a>
                                    </li>
                                </Show>
                            // <li>
                            // <a
                            // href=PageRoutes::ConfigViewer.path()
                            // class=move || active(PageRoutes::ConfigViewer.path())
                            // >
                            // Config
                            // </a>
                            // </li>
                            </ul>
                        </li>
                    </ul>
                </nav>
            </div>
        </div>
    }
}
