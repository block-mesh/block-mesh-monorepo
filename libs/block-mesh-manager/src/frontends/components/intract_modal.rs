use crate::frontends::components::icons::intract_icon::IntractIcon;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::utils::perk_util::sync_perk;
use block_mesh_common::constants::BUTTON_CLASS;
use block_mesh_common::interfaces::server_api::AuthStatusResponse;
use leptos::*;

#[component]
pub fn IntractModal() -> impl IntoView {
    let notifications = expect_context::<NotificationContext>();
    let auth_status = use_context::<AuthStatusResponse>();
    let email = RwSignal::new("".to_string());
    if let Some(a) = auth_status {
        email.set(a.email.clone().unwrap_or_default());
    }
    let sync_intract = create_action(move |_: &()| async move {
        if email.get_untracked().is_empty() {
            return;
        }
        match sync_perk().await {
            Ok(response) => {
                if response.error {
                    notifications.set_error(format!(
                        "Error from Intract: {}",
                        response.message.unwrap_or_default()
                    ));
                } else if response.cached {
                    notifications.set_success("Please try again in 10min");
                } else {
                    notifications.set_success("Intract synced");
                }
            }
            Err(_) => {
                notifications.set_error(format!(
                    "Failed to sync Intract , please ensure your Intract email is {}",
                    email.get_untracked().to_lowercase()
                ));
            }
        }
    });
    view! {
        <div class="bg-dark-blue">
            <div class="m-2">
                <div class="mb-4 rounded px-8 pb-8 pt-6 shadow-md bg-dark-blue">
                    <div class="flex flex-col gap-4">
                        <div>
                            <h2 class="text-2xl font-bold text-off-white mb-4">Instructions</h2>
                            <ol class="list-decimal list-inside space-y-2 text-off-white">
                                <li>
                                    <a
                                        href="https://quest.intract.io/project/6532e81854ff44c8a3b2c1d58dd68bd3"
                                        target="_blank"
                                        class="text-magenta"
                                    >
                                        Go to BlockMesh Network - Intract Page
                                    </a>
                                </li>
                                <li>
                                    Complete any of the tasks you want, there are no mandatory tasks
                                </li>
                                <li>Come back here</li>
                                <li>
                                    Click
                                    <button
                                        on:click=move |_| sync_intract.dispatch(())
                                        class=BUTTON_CLASS
                                    >
                                        <IntractIcon/>
                                        "Verify"
                                    </button>
                                </li>
                            </ol>
                        </div>
                        <hr class="border-t border-white mb-4"/>
                        <h2 class="text-2xl font-bold text-off-white mb-4">More Info</h2>
                        <ol class="list-disc list-inside space-y-2 text-off-white">
                            <li>
                                <a
                                    href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/INTRACT_PERK.md#how-to-fix-no-user-found-for-email-xyz"
                                    target="_blank"
                                    class="text-magenta"
                                >
                                    No user found for Email XYZ
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/INTRACT_PERK.md#how-to-fix-user-doesnt-belong-to-this-enterprise"
                                    target="_blank"
                                    class="text-magenta"
                                >
                                    User doesnt belong to this enterprise
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/block-mesh/block-mesh-support-faq/blob/main/INTRACT_PERK.md"
                                    target="_blank"
                                    class="text-magenta"
                                >
                                    FAQ
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://blockmesh.substack.com/p/blockmesh-x-intract"
                                    target="_blank"
                                    class="text-magenta"
                                >
                                    BlockMesh x Intract
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://persona.intract.io/proof-of-humanity"
                                    target="_blank"
                                    class="text-magenta"
                                >
                                    Proof of Humanity
                                </a>
                            </li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
    }
}
