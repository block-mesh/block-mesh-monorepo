use crate::frontends::context::notification_context::NotificationContext;
use block_mesh_common::interfaces::server_api::{AuthStatusResponse, EditEmailForm};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use reqwest::Client;

#[component]
pub fn EditEmail() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let auth_status = use_context::<AuthStatusResponse>();
    let new_email = RwSignal::new("".to_string());

    let email = RwSignal::new("".to_string());
    if let Some(a) = auth_status {
        email.set(a.email.clone().unwrap_or_default());
    }

    let submit = create_action(move |_| async move {
        if email.get_untracked().is_empty() {
            notifications.set_error("Please fill an email address");
            return;
        }

        let origin = window().origin();
        let client = Client::new();

        let response = client
            .post(format!("{}{}", origin, RoutesEnum::Static_Auth_Edit_Email))
            .form(&EditEmailForm {
                new_email: new_email.get_untracked(),
            })
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    notifications.set_error("Failed to send confirmation email");
                } else {
                    email.set(new_email.get_untracked());
                    notifications.set_success("Confirmation email sent");
                }
            }
            Err(_) => notifications.set_error("Failed to update email"),
        }
    });

    view! {
        <div class="bg-dark-blue">
            <form action="/edit_email" method="post" on:submit=|ev| ev.prevent_default()>
                <div class="m-2">
                    <div class="mb-4 rounded px-8 pb-8 pt-6 shadow-md bg-dark-blue">
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue mb-2 block text-sm font-bold text-off-white"
                                for="current_email"
                            >
                                Current Email
                            </label>
                            <input
                                class="w-full appearance-none rounded border px-3 py-2 text-black shadow text-black"
                                id="current_email"
                                type="text"
                                name="current_email"
                                disabled
                                value=move || { email.get() }
                                placeholder="Current Email"
                            />
                        </div>
                        <div class="mb-4">
                            <label
                                class="font-bebas-neue mb-2 block text-sm font-bold text-off-white"
                                for="new_email"
                            >
                                New Email
                            </label>
                            <input
                                class="w-full appearance-none rounded border px-3 py-2 text-black shadow"
                                id="new_email"
                                type="text"
                                name="new_email"
                                required
                                placeholder="New Invite Code"
                                on:keyup=move |ev: ev::KeyboardEvent| {
                                    let val = event_target_value(&ev);
                                    new_email.update(|v| *v = val);
                                }

                                on:change=move |ev| {
                                    let val = event_target_value(&ev);
                                    new_email.update(|v| *v = val);
                                }
                            />

                        </div>
                        <button
                            class="hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                            type="submit"
                            on:click=move |_| submit.dispatch(())
                        >
                            Update Email
                        </button>
                    </div>
                </div>
            </form>
        </div>
    }
}
