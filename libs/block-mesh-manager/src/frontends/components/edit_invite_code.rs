use crate::frontends::context::notification_context::NotificationContext;
use block_mesh_common::interfaces::server_api::{DashboardResponse, EditInviteCodeForm};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use reqwest::Client;

#[component]
pub fn EditInviteCode() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let async_data = use_context::<DashboardResponse>();
    let invite_code = RwSignal::new("".to_string());

    if let Some(data) = async_data {
        invite_code.set(data.invite_code);
    }

    let new_invite_code = create_rw_signal(String::default());

    let submit = create_action(move |_| async move {
        if invite_code.get_untracked().is_empty() {
            notifications.set_error("Please fill an invite code");
            return;
        }

        let origin = window().origin();
        let client = Client::new();

        let response = client
            .post(format!("{}{}", origin, RoutesEnum::Static_Auth_Edit_Invite))
            .form(&EditInviteCodeForm {
                new_invite_code: new_invite_code.get_untracked(),
            })
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().as_u16() != 200 {
                    notifications.set_error(
                        "Failed to update invite code, dont use spaces or special chars",
                    );
                } else {
                    invite_code.set(new_invite_code.get_untracked());
                    notifications.set_success("Invite code updated");
                }
            }
            Err(_) => notifications
                .set_error("Failed to update invite code, dont use spaces or special chars"),
        }
    });

    view! {
        <div class="bg-dark-blue">
            <form action="/edit_invite_code" method="post" on:submit=|ev| ev.prevent_default()>
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
    }
}
