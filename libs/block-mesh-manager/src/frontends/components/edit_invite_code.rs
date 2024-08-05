use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::frontend_webserver::components::error::ErrorComponent;
use crate::frontends::frontend_webserver::components::notification::NotificationComponent;
use block_mesh_common::interfaces::server_api::EditInviteCodeForm;
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::Suspense;
use leptos::*;
use reqwest::Client;

#[component]
pub fn EditInviteCode() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let async_data = WebAppContext::get_dashboard_data();
    let logged_in = WebAppContext::is_logged_in();
    let invite_code = create_rw_signal(String::default());
    let new_invite_code = create_rw_signal(String::default());

    create_effect(move |_| match async_data.get() {
        Some(Some(response)) => invite_code.set(response.invite_code.clone()),
        _ => invite_code.set("".to_string()),
    });
    let submit = create_action(move |_| async move {
        if invite_code.get_untracked().is_empty() {
            notifications.set_error("Please fill an invite code");
            return;
        }
        let origin = window().origin();
        let client = Client::new();
        let response = client
            .post(format!("{}/edit_invite_code", origin))
            .form(&EditInviteCodeForm {
                new_invite_code: new_invite_code.get_untracked(),
            })
            .send()
            .await;
        match response {
            Ok(_) => {
                invite_code.set(new_invite_code.get_untracked());
                notifications.set_success("Invite code updates");
            }
            Err(_) => notifications.set_error("Failed to update invite code"),
        }
    });

    view! {
        <Suspense fallback=move || {
            view! {
                <NotificationComponent
                    summary="Loading...".to_string()
                    detailed="Please wait while we load the dashboard".to_string()
                    go_to=RoutesEnum::Static_UnAuth_Login.to_string()
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
                            go_to=RoutesEnum::Static_UnAuth_Login.to_string()
                        />
                    }
                }
            >

                <div class="bg-dark-blue">
                    <form
                        action="/edit_invite_code"
                        method="post"
                        on:submit=|ev| ev.prevent_default()
                    >
                        <div class="m-2">
                            <div class="mb-4 rounded px-8 pb-8 pt-6 shadow-md bg-dark-blue">
                                <div class="mb-4">
                                    <label
                                        class="font-bebas-neue mb-2 block text-sm font-bold text-off-white"
                                        for="current_invite_code"
                                    >
                                        Current Invite
                                        Code
                                    </label>
                                    <input
                                        class="w-full appearance-none rounded border px-3 py-2 text-black shadow text-black"
                                        id="current_invite_code"
                                        type="text"
                                        name="current_invite_code"
                                        disabled
                                        value=move || { invite_code.get() }

                                        placeholder="Current Invite Code"
                                    />
                                </div>
                                <div class="mb-4">
                                    <label
                                        class="font-bebas-neue mb-2 block text-sm font-bold text-off-white"
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
                                        on:keyup=move |ev: ev::KeyboardEvent| {
                                            let val = event_target_value(&ev);
                                            new_invite_code.update(|v| *v = val);
                                        }

                                        on:change=move |ev| {
                                            let val = event_target_value(&ev);
                                            new_invite_code.update(|v| *v = val);
                                        }
                                    />

                                </div>
                                <button
                                    class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                                    type="submit"
                                    on:click=move |_| submit.dispatch(())
                                >
                                    Create New Invite Code
                                </button>
                            </div>
                        </div>
                    </form>
                </div>
            </Show>
        </Suspense>
    }
}
