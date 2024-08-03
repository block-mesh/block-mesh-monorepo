use leptos::*;

#[component]
pub fn Modal(show: RwSignal<bool>, children: Children) -> impl IntoView {
    let classes_1 = Signal::derive(move || {
        if show.get() {
            "ease-out duration-300 opacity-100"
        } else {
            "ease-out duration-300 opacity-0"
        }
    });
    let classes_2 = Signal::derive(move || {
        if show.get() {
            "ease-out duration-300 opacity-100 translate-y-0 sm:scale-100"
        } else {
            "ease-out duration-300 opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
        }
    });
    let z_index = Signal::derive(move || if show.get() { "z-100" } else { "-z-10" });

    view! {
        <div
            class=move || format!("relative {}", z_index.get())
            aria-labelledby="modal-title"
            role="dialog"
            aria-modal="true"
        >
            <div
                class=move || {
                    format!(
                        "fixed inset-0 bg-dark-blue bg-opacity-75 transition-opacity {}",
                        classes_1.get(),
                    )
                }

                aria-hidden="true"
            ></div>
            <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                    <div class=move || {
                        format!(
                            "border border-cyan relative transform overflow-hidden rounded-lg bg-dark-blue px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6 {}",
                            classes_2.get(),
                        )
                    }>
                        <div>{children()}</div>
                        <div class="mt-5 sm:mt-6">
                            <button
                                type="button"
                                on:click=move |_| {
                                    show.set(false);
                                }

                                class="border border-orange inline-flex w-full justify-center rounded-md  px-3 py-2 text-sm font-semibold text-off-white shadow-sm hover:bg-orange focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-orange"
                            >
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
