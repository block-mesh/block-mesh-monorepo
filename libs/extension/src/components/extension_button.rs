use leptos::*;

#[component]
pub fn ExtensionButton(text: String, mode: String) -> impl IntoView {
    let mode = Signal::derive(move || mode.clone());

    let class = move || {
        let base = "font-jetbrains auth-card-button items-center appearance-none bg-transparent rounded-md box-border text-white cursor-pointer inline-flex justify-center leading-none overflow-hidden w-6/12 relative text-left no-underline select-none whitespace-nowrap text-lg list-none py-2.5 transition-all duration-150 select-none touch-manipulation hover:shadow-custom hover:-translate-y-0.5 transition focus-visible:outline-blue";
        let base = format!(
            "{} {}",
            base,
            if mode.get() == "fit" {
                "p-2.5 mx-1"
            } else {
                ""
            }
        );
        base
    };

    let style = move || {
        format!("border: '2px solid var(--darkBlue)'; boxShadow: '0 2px 5px #2f898588, rgba(30, 45, 44, 0.5) 0 -3px 0 inset', willChange: 'box-shadow, transform' ; width: {}",
                if mode.get() == "fit" {
                    "fit-content"
                } else {
                    ""
                },
                )
    };

    view! {
        <button class=class style=style>
            {text}
        </button>
    }
}
