use leptos::*;

#[component]
pub fn Heading(children: Children) -> impl IntoView {
    view! {
        <h1 class="text-2xl/8 font-semibold text-zinc-950 sm:text-xl/8 dark:text-white">
            {children()}
        </h1>
    }
}
