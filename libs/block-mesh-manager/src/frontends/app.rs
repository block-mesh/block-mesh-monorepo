use crate::frontends::frontend_extension::extension_header::ExtensionServerHeader;
use crate::frontends::frontend_extension::pages::extension_wrapper::ExtensionWrapper;
use crate::frontends::frontend_extension::pages::logged_in::ExtensionLoggedIn;
use crate::frontends::frontend_extension::pages::login::ExtensionLogin;
use crate::frontends::frontend_extension::pages::register::ExtensionRegister;
use crate::frontends::frontend_webserver::components::notification_popup::NotificationPopupComponent;
use crate::frontends::frontend_webserver::context::webapp_context::WebAppContext;
use crate::frontends::frontend_webserver::pages::dashboard_page::DashboardPage;
use crate::frontends::frontend_webserver::pages::edit_invite_code_page::EditInvitePage;
use crate::frontends::frontend_webserver::pages::login_page::LoginPage;
use crate::frontends::frontend_webserver::pages::new_password_page::NewPasswordPage;
use crate::frontends::frontend_webserver::pages::register_page::RegisterPage;
use crate::frontends::frontend_webserver::pages::resend_confirmation_email_page::ResendConfirmationEmailPage;
use crate::frontends::frontend_webserver::pages::reset_password_page::ResetPasswordPage;
use crate::frontends::frontend_webserver::webserver_header::WebServerHeader;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router fallback=|| { view! { <p>Error</p> }.into_view() }>
            <Routes>
                <Route
                    path="/ui"
                    view=move || {
                        provide_context(WebAppContext::default());
                        view! {
                            <WebServerHeader/>
                            <NotificationPopupComponent/>
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
                                <ExtensionWrapper>
                                    <ExtensionLogin/>
                                </ExtensionWrapper>
                            }
                        }
                    />
                    <Route
                        path="/register"
                        view=move || {
                            view! {
                                <ExtensionWrapper>
                                    <ExtensionRegister/>
                                </ExtensionWrapper>
                            }
                        }
                    />
                    <Route
                        path="/logged_in"
                        view=move || {
                            view! {
                                <ExtensionWrapper>
                                    <ExtensionLoggedIn/>
                                </ExtensionWrapper>
                            }
                        }
                    />
                </Route>
            </Routes>
        </Router>
    }
}
