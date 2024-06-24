use crate::frontend::components::navbar::NavbarComponent;
use crate::frontend::context::webapp_context::WebAppContext;
use leptos::Suspense;
use leptos::*;

#[component]
pub fn EditInvitePage() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let logged_in = WebAppContext::is_logged_in();

    view! {
        <Suspense fallback=|| {
            view! { <p class="text-white">Loading...</p> }
        }>
            <Show
                when=move || {
                    match logged_in.get() {
                        Some(Some(response)) => response.logged_in,
                        _ => false,
                    }
                }

                fallback=|| {
                    view! { <p class="text-white">Not logged in</p> }
                }
            >

                <NavbarComponent/>

                <form action="/edit_invite_code" method="post">
                    <div class="m-2">
                        <div class="mb-4 rounded px-8 pb-8 pt-6 shadow-md bg-gray-800  border-white border-solid border-2">
                            <div class="mb-4">
                                <label
                                    class="mb-2 block text-sm font-bold text-white"
                                    for="current_invite_code"
                                >
                                    Current Invite
                                    Code
                                </label>
                                <input
                                    class="w-full appearance-none rounded border px-3 py-2 text-black shadow"
                                    id="current_invite_code"
                                    type="text"
                                    name="current_invite_code"
                                    disabled
                                    value=move || {
                                        match async_data.get() {
                                            Some(Some(response)) => response.invite_code.clone(),
                                            _ => "".to_string(),
                                        }
                                    }

                                    placeholder="Current Invite Code"
                                />
                            </div>
                            <div class="mb-4">
                                <label
                                    class="mb-2 block text-sm font-bold text-white"
                                    for="new_invite_code"
                                >
                                    New Invite Code
                                </label>
                                <input
                                    class="w-full appearance-none rounded border px-3 py-2 text-black shadow"
                                    id="new_invite_code"
                                    type="text"
                                    name="new_invite_code"
                                    required
                                    placeholder="New Invite Code"
                                />
                            </div>
                            <button
                                class="focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none"
                                type="submit"
                            >
                                Create New Invite Code
                            </button>
                        </div>
                    </div>
                </form>
            </Show>
        </Suspense>
    }
}
