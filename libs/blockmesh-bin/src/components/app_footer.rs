use leptos::*;
use leptos_router::A;

#[component]
pub fn AppFooter() -> impl IntoView {
    view! {
        <footer class="bg-gray-800 text-white py-6 border-t-2 border-white">
            <div class="w-full flex flex-col items-center md:flex-row md:justify-between px-4">
                <div class="text-center md:text-left"></div>
                <div class="mt-4 md:mt-0">
                    <h5 class="text-gray-400 hover:text-white mx-2">BlockMesh Network</h5>
                    <A
                        href="https://x.com/blockmesh_xyz"
                        target="_blank"
                        class="text-gray-400 hover:text-white mx-2"
                    >
                        Contact Us
                    </A>
                </div>
            </div>
        </footer>
    }
}
