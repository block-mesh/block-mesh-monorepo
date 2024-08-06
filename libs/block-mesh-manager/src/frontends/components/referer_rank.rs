use leptos::*;
use tailwind_fuse::tw_join;

#[component]
pub fn RefererRank(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] step: u8,
    #[prop(into, optional)] is_complete: bool,
) -> impl IntoView {
    let class = tw_join!(
        "absolute left-0 top-0 w-1 lg:bottom-0 lg:top-auto lg:w-full",
        is_complete.then_some("bg-blue shadow-light h-px")
    );

    view! {
        <li class="relative lg:flex-1 refer-card">
            <div class="rounded-t-md border border-b-0 border-gray-200 lg:border-0">
                <div class="refer-card-wrapper">
                    <span class=class aria-hidden="true"></span>
                    <span class="flex items-start px-6 py-5 text-sm font-medium">
                        <span class="flex-shrink-0">
                            <Show
                                when=move || is_complete
                                fallback=move || {
                                    view! {
                                        <span class="flex h-10 w-10 items-center justify-center rounded-full border-2 border-orange">
                                            <span class="text-off-white">{step.to_string()}</span>
                                        </span>
                                    }
                                }
                            >

                                <span class="flex h-10 w-10 items-center justify-center rounded-full bg-blue">
                                    <svg
                                        class="h-6 w-6 text-dark"
                                        viewBox="0 0 24 24"
                                        fill="currentColor"
                                        aria-hidden="true"
                                    >
                                        <path
                                            fillRule="evenodd"
                                            d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z"
                                            clipRule="evenodd"
                                        ></path>
                                    </svg>
                                </span>
                            </Show>

                        </span>

                        <span class="ml-4 mt-0.5 flex min-w-0 flex-col">
                            <span class="font-bebas-neue text-sm font-medium text-orange">
                                {title}
                            </span>
                            <span class="font-open-sans text-sm font-medium text-off-white">
                                {description}
                            </span>
                        </span>
                    </span>
                </div>
                <Show when=move || step != 1>
                    <div class="absolute inset-0 top-0 hidden w-3 lg:block" aria-hidden="true">
                        <svg
                            class="h-full w-full text-off-white"
                            viewBox="0 0 12 82"
                            fill="none"
                            preserveAspectRatio="none"
                        >
                            <path
                                d="M0.5 0V31L10.5 41L0.5 51V82"
                                stroke="currentcolor"
                                vectorEffect="non-scaling-stroke"
                            ></path>
                        </svg>
                    </div>
                </Show>
            </div>
        </li>
    }
}
