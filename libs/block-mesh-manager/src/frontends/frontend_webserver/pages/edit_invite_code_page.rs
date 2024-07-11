use crate::frontends::frontend_webserver::components::error::ErrorComponent;
use crate::frontends::frontend_webserver::components::navbar::NavbarComponent;
use crate::frontends::frontend_webserver::components::notification::NotificationComponent;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use leptos::Suspense;
use leptos::*;

#[component]
pub fn EditInvitePage() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    let logged_in = WebAppContext::is_logged_in();
    view! {
        <Suspense fallback=move || {
            view! {
                <NotificationComponent
                    summary="Loading...".to_string()
                    detailed="Please wait while we load the dashboard".to_string()
                    go_to="/ui/login".to_string()
                />
            }
        }>
            <Show
                when=move || {
                    match logged_in.get() {
                        Some(Some(response)) => response.logged_in,
                        _ => false,
                    }
                }

                fallback=|| {
                    view! {
                        <ErrorComponent
                            code=401
                            summary="Not Logged In".to_string()
                            detailed="You must be logged in to view this page".to_string()
                            go_to="/ui/login".to_string()
                        />
                    }
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
