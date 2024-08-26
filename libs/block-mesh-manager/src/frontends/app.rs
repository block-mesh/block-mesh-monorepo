use crate::frontends::common_header::CommonHeader;
use crate::frontends::components::notification_popup::NotificationPopup;
use crate::frontends::context::auth_context::AuthContext;
use crate::frontends::context::extension_state::ExtensionContext;
use crate::frontends::context::notification_context::NotificationContext;
use crate::frontends::context::reload_context::ReloadContext;
use crate::frontends::context::size_context::SizeContext;
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
use crate::frontends::frontend_webserver::webserver_header::WebServerHeader;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use crate::frontends::new_frontend_webserver::app::new_dashboard::NewDashboard;
use crate::frontends::new_frontend_webserver::app::perks::Perks;
use crate::frontends::new_frontend_webserver::app::referrals::Referrals;
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
    provide_context(SizeContext::default());
    provide_context(ReloadContext::default());

    let extension_state = use_context::<ExtensionContext>().unwrap();
    let auth_state = use_context::<AuthContext>().unwrap();
    let extension_resource = ExtensionContext::init_resource(extension_state);
    let none_extension_resource = None::<Resource<(), ExtensionContext>>;
    let auth_state = AuthContext::init_as_resource(auth_state);

    view! {
        <CommonHeader />
        <Router fallback=|| { view! { <p>Error</p> }.into_view() }>
            <Routes>
                <Route
                    path="/ui"
                    view=move || {
                        view! {
                            <WebServerHeader />
                            <NotificationPopup />

                            <ApplicationLayout>
                                <Outlet />
                            </ApplicationLayout>
                        }
                    }
                >
                    <Route path="/dashboard" view=NewDashboard />
                    <Route path="/referrals" view=Referrals />
                    <Route path="/perks" view=Perks />
                </Route>
                <Route
                    path="/tauri"
                    view=move || {
                        view! {
                            <TauriHeader />
                            <NotificationPopup />
                            <Outlet />
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
                                    loading=|| view! { <TauriLoading /> }
                                    class=""
                                >
                                    <TauriNavigator />
                                    <TauriLogin />
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
                                    loading=|| view! { <TauriLoading /> }
                                    class=""
                                >
                                    <TauriNavigator />
                                    <TauriRegister />
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
                                    loading=|| view! { <TauriLoading /> }
                                    class=""
                                >
                                    <TauriNavigator />
                                    <TauriLoggedIn />
                                </Wrapper>
                            }
                        }
                    />

                </Route>
                <Route
                    path="/ext"
                    view=move || {
                        view! {
                            <ExtensionServerHeader />
                            <Outlet />
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
                                    loading=|| view! { <ExtensionLoading /> }
                                >
                                    <NotificationPopup />
                                    <ExtensionNavigator />
                                    <ExtensionLogin />
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
                                    loading=|| view! { <ExtensionLoading /> }
                                >
                                    <NotificationPopup />
                                    <ExtensionNavigator />
                                    <ExtensionRegister />
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
                                    loading=|| view! { <ExtensionLoading /> }
                                >
                                    <NotificationPopup />
                                    <ExtensionNavigator />
                                    <ExtensionLoggedIn />
                                </Wrapper>
                            }
                        }
                    />

                </Route>
            </Routes>
        </Router>
    }
}
