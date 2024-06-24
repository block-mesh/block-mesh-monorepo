use block_mesh_common::constants::BLOCK_MESH_LOGO;
use leptos::*;
use leptos_router::A;

#[component]
pub fn NavbarComponent() -> impl IntoView {
    let (b1, set_b1) = create_signal("block");
    let (b2, set_b2) = create_signal("hidden");
    let (menu, set_menu) = create_signal("hidden");
    let click = move || {
        if b1.get() == "block" {
            set_b1.set("hidden");
            set_b2.set("block");
            set_menu.set("block");
        } else {
            set_b1.set("block");
            set_b2.set("hidden");
            set_menu.set("hidden");
        }
    };

    view! {
        <nav class="bg-gray-800" id="top-navbar">
            <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                    <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                        <button
                            type="button"
                            on:click=move |_| click()
                            class="relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                        >
                            <span class="absolute -inset-0.5"></span>
                            <span class="sr-only">Open main menu</span>
                            <svg
                                id="toggle-menu-button-1"
                                class=move || format!("{} h-6 w-6", b1.get())
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                                ></path>
                            </svg>
                            <svg
                                id="toggle-menu-button-2"
                                class=move || format!("{} h-6 w-6", b2.get())
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                aria-hidden="true"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M6 18L18 6M6 6l12 12"
                                ></path>
                            </svg>
                        </button>
                    </div>
                    <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
                        <div class="flex flex-shrink-0 items-center">
                            <img
                                class="h-8 w-auto"
                                src=move || BLOCK_MESH_LOGO
                                alt="BlockMesh Network"
                            />
                        </div>
                        <div class="hidden sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                                <A
                                    href="/ui/dashboard"
                                    class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                                >
                                    Dashboard
                                </A>
                                <A
                                    href="/ui/tasks_table"
                                    class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                                >
                                    Tasks
                                </A>
                                <A
                                    href="/logout"
                                    class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                                >
                                    Logout
                                </A>
                            </div>
                        </div>
                    </div>
                    // <!-- Notifications button -->
                    // <!-- Profile dropdown -->
                    <div class="absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0"></div>
                </div>
            </div>

            <div class=move || format!("{} sm:hidden", menu.get()) id="mobile-menu">
                <div class="space-y-1 px-2 pb-3 pt-2">
                    <A
                        href="/ui/dashboard"
                        class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                    >
                        Dashboard
                    </A>
                    <A
                        href="/ui/tasks_table"
                        class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                    >
                        Tasks
                    </A>
                    <A
                        href="/logout"
                        class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-gray-700 hover:text-white"
                    >
                        Logout
                    </A>
                </div>
            </div>
        </nav>
    }
}
