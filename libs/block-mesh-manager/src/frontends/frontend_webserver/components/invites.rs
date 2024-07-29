use leptos::*;
use leptos_router::A;

use crate::frontends::frontend_webserver::components::icons::clipboard_icon::ClipboardIcon;
use crate::frontends::frontend_webserver::components::icons::edit_icon::EditIcon;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use crate::frontends::components::icons::clipboard_icon::ClipboardIcon;
use crate::frontends::components::icons::edit_icon::EditIcon;
use crate::frontends::context::webapp_context::WebAppContext;
use leptos::*;
use leptos_router::A;

#[component]
pub fn InvitesComponent() -> impl IntoView {
    let async_data = WebAppContext::get_dashboard_data();

    fn get_invite_code() -> Option<String> {
        let doc = document();
        let el = match doc.get_element_by_id("copy_invite_code") {
            None => return None,
            Some(el) => el,
        };
        el.get_attribute("invite_code")
    }

    let copy_to_clipboard = move |_| {
        #[cfg(all(web_sys_unstable_apis, feature = "hydrate"))]
        {
            use crate::frontends::context::notification_context::NotificationContext;
            let notifications = expect_context::<NotificationContext>();
            if let Some(clipboard) = web_sys::window().unwrap().navigator().clipboard() {
                if let Some(invite_url_string) = get_invite_code() {
                    let _ = clipboard.write_text(&format!(
                        "https://app.blockmesh.xyz/register?invite_code={}",
                        invite_url_string
                    ));
                    notifications.set_success("Successfully Copied");
                } else {
                    notifications.set_error("Failed to copy invite code");
                }
            }
        }
        #[cfg(not(web_sys_unstable_apis))]
        {}
    };

    view! {
        <div class="border-off-white border m-2 relative overflow-hidden rounded-[30px] pt-6 md:pt-[33px] pb-7 md:pb-[39px] pl-[11px] md:pl-[44px]">
            <div class="grid grid-cols-1 sm:grid-cols-3 lg:grid-cols-3 rounded">
                <div class="py-6">
                    <p class="font-bebas-neue text-sm font-medium leading-6 text-off-white">
                        Invite Code
                    </p>
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

                            on:click=copy_to_clipboard
                            class="inline-flex hover:text-orange text-off-white py-2 px-4 border border-orange rounded font-bebas-neue focus:outline-none focus:shadow-outline"
                        >
                            <ClipboardIcon/>
                        </button>
                    </p>
                </div>
                <div class="py-6 px-4 sm:px-6 lg:px-8">
                    <p class="font-bebas-neue text-sm font-medium leading-6 text-off-white">
                        Referrals
                    </p>
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
                    <p class="font-bebas-neue text-sm font-medium leading-6 text-off-white">
                        Edit
                    </p>
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
    }
}
