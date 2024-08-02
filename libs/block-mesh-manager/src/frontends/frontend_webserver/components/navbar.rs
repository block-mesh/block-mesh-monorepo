use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::utils::auth::connect_wallet;
use crate::frontends::utils::connectors::{pubkey, sign_message};
use block_mesh_common::constants::BLOCK_MESH_LOGO;
use block_mesh_common::interfaces::server_api::ConnectWalletRequest;
use js_sys::Uint8Array;
use leptos::*;
use leptos_router::A;
use uuid::Uuid;
use wasm_bindgen::JsValue;

#[component]
pub fn NavbarComponent() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let logged_in = WebAppContext::is_logged_in();
    let wallet = Signal::derive(move || {
        if let Some(Some(l)) = logged_in.get() {
            l.wallet_address
        } else {
            None
        }
    });
    let button_enable = Signal::derive(move || wallet.get().is_none());
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

    let click_button = move || {
        spawn_local(async move {
            if !button_enable.get() {
                notifications.set_error("Backpack already connected");
                return;
            }
            if !window().has_own_property(&JsValue::from_str("backpack")) {
                notifications.set_error("Backpack wallet not found");
            }
            let msg = Uuid::new_v4().to_string();
            let key = pubkey().await;
            let sign = sign_message(&msg).await;
            let uint8_array = Uint8Array::new(&sign);
            let mut vec = vec![0; uint8_array.length() as usize];
            uint8_array.copy_to(&mut vec[..]);
            let origin = window().origin();
            match connect_wallet(
                origin,
                ConnectWalletRequest {
                    pubkey: key.as_string().unwrap(),
                    message: msg.to_string(),
                    signature: vec,
                },
            )
            .await
            {
                Ok(_) => {
                    logged_in.refetch();
                    notifications.set_success("Connected successfully");
                }
                Err(_) => notifications.set_error("Failed to connect"),
            }
        })
    };

    view! {
        <nav class="bg-dark-blue" id="top-navbar">
            <div class="mx-auto max-w-7xl px-2 sm:px-6 lg:px-8">
                <div class="relative flex h-16 items-center justify-between">
                    <div class="absolute inset-y-0 left-0 flex items-center sm:hidden">
                        <button
                            type="button"
                            on:click=move |_| click()
                            class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"

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
                            <a href="/">
                                <img
                                    class="h-8 w-auto"
                                    src=move || BLOCK_MESH_LOGO
                                    alt="BlockMesh Network"
                                />
                            </a>
                        </div>
                        <div class="hidden sm:ml-6 sm:block">
                            <div class="flex space-x-4">
                                <A
                                    href="/ui/dashboard"
                                    class="rounded-md px-3 py-2 font-bebas-neue mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:bg-gray-700 hover:text-orange"
                                >
                                    Dashboard
                                </A>
                                <A
                                    href="/logout"
                                    class="rounded-md px-3 py-2 font-bebas-neue mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:bg-gray-700 hover:text-orange"
                                >
                                    Logout
                                </A>
                                <button
                                    on:click=move |_| click_button()
                                    class="rounded-md px-3 py-2 font-bebas-neue mb-2 inline-block align-baseline font-bold text-xs text-cyan hover:bg-gray-700 hover:text-orange"
                                >
                                    {move || {
                                        if button_enable.get() {
                                            "Connect Wallet"
                                        } else {
                                            "Wallet Connected"
                                        }
                                    }}

                                </button>
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
                        class="block rounded-md px-3 py-2 font-bebas-neue mb-2 align-baseline font-bold text-xs text-cyan hover:bg-gray-700 hover:text-orange"
                    >
                        Dashboard
                    </A>
                    <A
                        href="/logout"
                        class="block rounded-md px-3 py-2 font-bebas-neue mb-2 align-baseline font-bold text-xs text-cyan hover:bg-gray-700 hover:text-orange"
                    >
                        Logout
                    </A>
                </div>
            </div>
        </nav>
    }
}
