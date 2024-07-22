use super::application_layout::ApplicationLayout;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn RootLayout(children: Children) -> impl IntoView {
    provide_meta_context();

    view! {
        <Html
            lang="en"
            class="text-zinc-950 antialiased lg:bg-zinc-100 dark:bg-zinc-900 dark:text-white dark:lg:bg-zinc-950"
        />
        <Link rel="preconnect" href="https://rsms.me/"/>
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css"/>
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0"
        />

        <ApplicationLayout>{children()}</ApplicationLayout>
    }
}
