use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn ExtensionButton(#[prop(optional)] fit: bool, #[prop(into)] text: String) -> impl IntoView {
    let class = tw_join!(
        "font-jetbrains auth-card-button items-center appearance-none bg-transparent rounded-md box-border text-white cursor-pointer inline-flex justify-center leading-none overflow-hidden w-6/12 relative text-left no-underline select-none whitespace-nowrap text-lg list-none py-2.5 transition-all duration-150 select-none touch-manipulation hover:shadow-custom hover:-translate-y-0.5 transition focus-visible:outline-blue",
        fit.then_some("p-2.5 mx-1"),
    );

    view! {
        <button
            class=class
            style="border: 2px solid var(--darkBlue); box-shadow: 0 2px 5px #2f898588, rgba(30, 45, 44, 0.5) 0 -3px 0 inset; will-change: box-shadow, transform;"
            style:width=fit.then_some("fit-content")
        >
            {text}
        </button>
    }
}
