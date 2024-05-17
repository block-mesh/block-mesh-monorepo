use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div
            class="relative h-screen bg-auto bg-no-repeat bg-center"
            style="background-image: url('https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public');"
        >
            <div class="relative z-10 flex flex-col items-center justify-center h-full text-center text-white">
                <h1 class="text-4xl font-bold md:text-6xl">BlockMesh Network</h1>
            </div>

        </div>
    }
}
