use leptos::*;
use crate::frontends::context::reload_context::ReloadContext;

#[component]
pub fn ReloadButton() -> impl IntoView {
    let reload = expect_context::<ReloadContext>();

    let on_click = move |_| {
        reload.trigger_reload();
    };

    view! {
        <div on:click=on_click class="hover:text-orange text-off-white cursor-pointer focus:outline-none flex-grow text-end opacity-50 hover:opacity-100">
             <span class="material-symbols-outlined">refresh</span>
        </div>
    }
}