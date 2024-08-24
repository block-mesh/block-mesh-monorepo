use block_mesh_common::constants::BLOCK_MESH_LOGO;
use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <img class="h-16 m-auto" src=BLOCK_MESH_LOGO alt="logo" />
        </div>
    }
}
