use leptos::*;

use crate::frontend::webserver_extension::connectors::{bla, get_storage_value};

#[component]
pub fn WebServerExtensionHomePage() -> impl IntoView {
    let action = create_action(move |_| async move {
        bla("ss").await;
        get_storage_value("blockmesh_url").await;
    });

    view! {
        <span class="text-white">WebServerExtensionHomePage</span>
        <button
            class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            on:click=move |_| action.dispatch(())
        >
            Get blockmesh_url from storage
        </button>
    }
}
