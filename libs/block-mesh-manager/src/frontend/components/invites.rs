use crate::frontend::components::icons::clipboard_icon::ClipboardIcon;
use crate::frontend::components::icons::edit_icon::EditIcon;
use crate::frontend::context::webapp_context::WebAppContext;
use leptos::*;
use leptos_meta::Script;
use leptos_router::A;

#[component]
pub fn InvitesComponent() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();
    view! {
        <div class="m-2">
            <Script>
                r#"
                function copy_invite_code_to_clipboard() {
                let button = document.getElementById("copy_invite_code");
                if (button) {
                    let copyText = button.getAttribute("invite_code")
                    let server = window.location.origin;
                    navigator.clipboard.writeText(`${server}/register?invite_code=${copyText}`);
                    // open_notification("Successfully Copied", "We have copied the invite link to your clipboard. Share it with your friends!");
                }
                
                }
                "#
            </Script>

            <div class="border-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
                <div class="grid grid-cols-1 sm:grid-cols-3 lg:grid-cols-3 rounded">
                    <div class="py-6">
                        <p class="text-sm font-medium leading-6 text-gray-400">Invite Code</p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                            <button
                                type="button"
                                id="copy_invite_code"
                                invite_code=move || {
                                    match async_data.get() {
                                        Some(Some(response)) => response.invite_code.clone(),
                                        _ => "".to_string(),
                                    }
                                }

                                onclick="copy_invite_code_to_clipboard()"
                                class="border border-white inline-flex items-center gap-x-1.5 rounded-md px-2.5 py-1.5 text-sm font-semibold text-white shadow-sm hover:bg-gray-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                            >
                                Copy Invite Link
                                <ClipboardIcon/>
                            </button>

                        </p>
                    </div>
                    <div class="py-6 px-4 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-gray-400">Users Invited</p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                            <span class="text-4xl font-semibold tracking-tight text-white">
                                {move || {
                                    let number_of_users_invited = match async_data.get() {
                                        Some(Some(response)) => response.number_of_users_invited,
                                        _ => 0i64,
                                    };
                                    format!("{}", number_of_users_invited)
                                }}

                            </span>
                        </p>
                    </div>
                    <div class="py-6 px-4 sm:px-6 lg:px-8">
                        <p class="text-sm font-medium leading-6 text-gray-400">Edit Invite Code</p>
                        <p class="mt-2 flex items-baseline gap-x-2">
                            <A
                                href="/ui/edit_invite_code"
                                class="text-4xl font-semibold tracking-tight text-white"
                            >
                                <EditIcon/>
                            </A>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
