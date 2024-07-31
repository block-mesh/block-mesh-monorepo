use crate::frontends::common_header::CommonHeader;
use crate::frontends::components::notification_popup::NotificationPopup;
use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::extension_state::ExtensionContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::size_context::SizeContext;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::frontend_extension::components::navigator::ExtensionNavigator;
use crate::frontends::frontend_extension::extension_header::ExtensionServerHeader;
use crate::frontends::frontend_extension::pages::loading::ExtensionLoading;
use crate::frontends::frontend_extension::pages::logged_in::ExtensionLoggedIn;
use crate::frontends::frontend_extension::pages::login::ExtensionLogin;
use crate::frontends::frontend_extension::pages::register::ExtensionRegister;
use crate::frontends::frontend_tauri::components::navigator::TauriNavigator;
use crate::frontends::frontend_tauri::pages::loading::TauriLoading;
use crate::frontends::frontend_tauri::pages::logged_in::TauriLoggedIn;
use crate::frontends::frontend_tauri::pages::login::TauriLogin;
use crate::frontends::frontend_tauri::pages::register::TauriRegister;
use crate::frontends::frontend_tauri::tauri_header::TauriHeader;
use crate::frontends::frontend_webserver::pages::dashboard_page::DashboardPage;
use crate::frontends::frontend_webserver::pages::edit_invite_code_page::EditInvitePage;
use crate::frontends::frontend_webserver::pages::login_page::LoginPage;
use crate::frontends::frontend_webserver::pages::new_password_page::NewPasswordPage;
use crate::frontends::frontend_webserver::pages::register_page::RegisterPage;
use crate::frontends::frontend_webserver::pages::resend_confirmation_email_page::ResendConfirmationEmailPage;
use crate::frontends::frontend_webserver::pages::reset_password_page::ResetPasswordPage;
use crate::frontends::frontend_webserver::webserver_header::WebServerHeader;
use crate::frontends::new_frontend_webserver::{Extension, Home, Orders};
use crate::frontends::wrapper::Wrapper;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(AuthContext::default());
    provide_context(NotificationContext::default());
    provide_context(ExtensionContext::default());
    provide_context(WebAppContext::default());
    provide_context(SizeContext::default());

    let none_resource: Option<Resource<(), ()>> = None;
    let _notification = use_context::<NotificationContext>().unwrap();
    let extension_state = use_context::<ExtensionContext>().unwrap();
    let auth_state = use_context::<AuthContext>().unwrap();
    let extension_resource = ExtensionContext::init_resource(extension_state);
    let none_extension_resource = None::<Resource<(), ExtensionContext>>;
    let auth_state = AuthContext::init_as_resource(auth_state);

    let new_server_class = "text-zinc-950 antialiased lg:bg-zinc-100 dark:bg-zinc-900 dark:text-white dark:lg:bg-zinc-950";

    view! {
        <CommonHeader/>
        <Script>
            r#"
                window.addEventListener("message", onMessage);
                function onMessage(e) {
                    if (window?.webkit?.messageHandlers?.ReactNativeWebView && !window.message_channel_port) {
                        window.message_channel_port = window?.webkit?.messageHandlers?.ReactNativeWebView;
                        window.message_channel_port.postMessage("READY");
                        return;
                    }
                    if (!e.ports.length) return;
                    e.ports[0].postMessage("READY");
                    window.message_channel_port = e.ports[0];
                    window.message_channel_port.onmessage = (msg) => {
                        // console.log("msg", window.location.href , msg, msg?.data);
                    }
                }
                // window.addEventListener("message", onMessage2);
                // function onMessage2(e) {
                //     window.webkit.messageHandlers.ReactNativeWebView.postMessage(`hello from server ${JSON.stringify(e)}`);
                // }
            "#
        </Script>
        <Router fallback=|| { view! { <p>Error</p> }.into_view() }>
            <Routes>
                <Route
                    path="/new"

                    view=move || {
                        view! {
                            <Wrapper
                                resource=none_resource
                                auth=none_resource
                                loading=|| view! { <p>Loading</p> }
                                class=new_server_class
                            >
                                <Home/>
                            </Wrapper>
                        }
                    }
                />

                <Route
                    path="/referer"

                    view=move || {
                        view! {
                            <Wrapper
                                resource=none_resource
                                auth=none_resource
                                loading=|| view! { <p>Loading</p> }
                                class=new_server_class
                            >
                                <Orders/>
                            </Wrapper>
                        }
                    }
                />

                <Route
                    path="/extension"

                    view=move || {
                        view! {
                            <Wrapper
                                resource=none_resource
                                auth=none_resource
                                loading=|| view! { <p>Loading</p> }
                                class=new_server_class
                            >
                                <Extension/>
                            </Wrapper>
                        }
                    }
                />

                <Route
                    path="/ui"
                    view=move || {
                        view! {
                            <WebServerHeader/>
                            <NotificationPopup/>
                            <Outlet/>
                        }
                    }
                >

                    <Route path="/login" view=LoginPage/>
                    <Route path="/reset_password" view=ResetPasswordPage/>
                    <Route path="/register" view=RegisterPage/>
                    <Route path="/dashboard" view=DashboardPage/>
                    <Route path="/resend_confirmation_email" view=ResendConfirmationEmailPage/>
                    <Route path="/new_password" view=NewPasswordPage/>
                    <Route path="/edit_invite_code" view=EditInvitePage/>
                    <Route
                        path="/new_dashboard"

                        view=move || {
                            view! {
                                <Wrapper
                                    resource=none_resource
                                    auth=none_resource
                                    loading=|| view! { <p>Loading</p> }
                                    class=new_server_class
                                >
                                    <Home/>
                                </Wrapper>
                            }
                        }
                    />

                    <Route
                        path="/referral"

                        view=move || {
                            view! {
                                <Wrapper
                                    resource=none_resource
                                    auth=none_resource
                                    loading=|| view! { <p>Loading</p> }
                                    class=new_server_class
                                >
                                    <Orders/>
                                </Wrapper>
                            }
                        }
                    />

                </Route>
                <Route
                    path="/tauri"
                    view=move || {
                        view! {
                            <TauriHeader/>
                            <NotificationPopup/>
                            <Outlet/>
                        }
                    }
                >

                    <Route
                        path="/login"
                        view=move || {
                            view! {
                                <Wrapper
                                    resource=none_extension_resource
                                    auth=Some(auth_state)
                                    loading=|| view! { <TauriLoading/> }
                                    class=""
                                >
                                    <TauriNavigator/>
                                    <TauriLogin/>
                                </Wrapper>
                            }
                        }
                    />

                    <Route
                        path="/register"
                        view=move || {
                            view! {
                                <Wrapper
                                    resource=none_extension_resource
                                    auth=Some(auth_state)
                                    loading=|| view! { <TauriLoading/> }
                                    class=""
                                >
                                    <TauriNavigator/>
                                    <TauriRegister/>
                                </Wrapper>
                            }
                        }
                    />

                    <Route
                        path="/logged_in"
                        view=move || {
                            view! {
                                <Wrapper
                                    resource=none_extension_resource
                                    auth=Some(auth_state)
                                    loading=|| view! { <TauriLoading/> }
                                    class=""
                                >
                                    <TauriNavigator/>
                                    <TauriLoggedIn/>
                                </Wrapper>
                            }
                        }
                    />

                </Route>
                <Route
                    path="/ext"
                    view=move || {
                        view! {
                            <ExtensionServerHeader/>
                            <Outlet/>
                        }
                    }
                >

                    <Route
                        path="/login"
                        view=move || {
                            view! {
                                <Wrapper
                                    resource=Some(extension_resource)
                                    auth=Some(auth_state)
                                    class=""
                                    loading=|| view! { <ExtensionLoading/> }
                                >
                                    <ExtensionNotifications/>
                                    <ExtensionNavigator/>
                                    <ExtensionLogin/>
                                </Wrapper>
                            }
                        }
                    />

                    <Route
                        path="/register"
                        view=move || {
                            view! {
                                <Wrapper
                                    class=""
                                    resource=Some(extension_resource)
                                    auth=Some(auth_state)
                                    loading=|| view! { <ExtensionLoading/> }
                                >
                                    <ExtensionNotifications/>
                                    <ExtensionNavigator/>
                                    <ExtensionRegister/>
                                </Wrapper>
                            }
                        }
                    />

                    <Route
                        path="/logged_in"
                        view=move || {
                            view! {
                                <Wrapper
                                    class=""
                                    resource=Some(extension_resource)
                                    auth=Some(auth_state)
                                    loading=|| view! { <ExtensionLoading/> }
                                >
                                    <ExtensionNotifications/>
                                    <ExtensionNavigator/>
                                    <ExtensionLoggedIn/>
                                </Wrapper>
                            }
                        }
                    />

                </Route>
            </Routes>
        </Router>
    }
}
