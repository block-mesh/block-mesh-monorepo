use crate::frontends::context::notification_context::NotificationContext;
use leptos::*;

#[component]
pub fn NotificationPopup() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let success = Signal::derive(move || notifications.success.get());
    let error = Signal::derive(move || notifications.error.get());
    let opacity = Signal::derive(move || {
        if notifications.success.get().is_some() || notifications.error.get().is_some() {
            "opacity-100"
        } else {
            "opacity-0"
        }
    });
    view! {
        <div
            aria-live="assertive"
            class="mt-16 pointer-events-none fixed inset-0 flex items-start px-4 py-6 sm:items-start sm:p-6"
        >
            <div class="flex w-full flex-col items-center space-y-4 sm:items-end">
                <div class=move || {
                    format!(
                        "pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg bg-off-white border-black border shadow-lg ring-1 ring-black ring-opacity-5 {}",
                        opacity.get(),
                    )
                }>

                    <div class="p-2">
                        <div class="flex items-start">
                            <div class="flex-shrink-0">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="1.5"
                                    stroke="currentColor"
                                    class="h-6 w-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="m11.25 11.25.041-.02a.75.75 0 0 1 1.063.852l-.708 2.836a.75.75 0 0 0 1.063.853l.041-.021M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-3.75h.008v.008H12V8.25Z"
                                    ></path>
                                </svg>
                            </div>
                            <div class="ml-3 w-0 flex-1 pt-0.5">
                                <p class="text-sm text-gray-500">
                                    {move || {
                                        error
                                            .get()
                                            .map(|err| {
                                                view! { <p style="color:red;">{err}</p> }
                                            })
                                    }}
                                    {move || {
                                        success
                                            .get()
                                            .map(|suc| {
                                                view! { <p style="color:green;">{suc}</p> }
                                            })
                                    }}

                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
