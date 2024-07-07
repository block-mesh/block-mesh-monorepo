use crate::frontend::components::notification_popup::NotificationPopupComponent;
use crate::frontend::context::webapp_context::WebAppContext;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::frontend::pages::dashboard_page::DashboardPage;
use crate::frontend::pages::edit_invite_code_page::EditInvitePage;
use crate::frontend::pages::login_page::LoginPage;
use crate::frontend::pages::new_password_page::NewPasswordPage;
use crate::frontend::pages::register_page::RegisterPage;
use crate::frontend::pages::resend_confirmation_email_page::ResendConfirmationEmailPage;
use crate::frontend::pages::reset_password_page::ResetPasswordPage;
use crate::frontend::webserver_extension::pages::home::WebServerExtensionHomePage;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_context(WebAppContext::default());

    view! {
        <Stylesheet href="https://r2-assets.blockmesh.xyz/tailwind.css"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Agbalumo&family=Varela+Round&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,500;1,600;1,700;1,800;1,900&display=swap"
            rel="stylesheet"
        />
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=Varela+Round&display=swap"
            rel="stylesheet"
        />
        <Title text="BlockMesh Network"/>
        <Link
            rel="icon"
            // type_=Some(i)
            href="https://imagedelivery.net/3RKw_J_fJQ_4KpJP3_YgXA/ebe1a44f-2f67-44f2-cdec-7f13632b7c00/public"
        />
        <Link
            rel="stylesheet"
            // type_=Some(c)
            href="https://r2-assets.blockmesh.xyz/tailwind.css"
        />
        <Style>
            r#"
                #content{
                  Position:static;
                  z-index: 1000000;
                }
            "#
        </Style>
        // async_=Some(t)
        <Script src="https://www.googletagmanager.com/gtag/js?id=G-RYHLW3MDK2"/>
        <Script>
            r#"
            window.dataLayer = window.dataLayer || [];
            function gtag() {
                dataLayer.push(arguments);
            }
            gtag('js', new Date());
            gtag('config', 'G-RYHLW3MDK2');
            "#
        </Script>
        <Script>
            r#"
            let port = null;
            function onPortMessge(e) {
                console.log("onPortMessage", e);
            }
            function onMessage(e) {
                if (!e.ports.length) return;
                e.ports[0].postMessage("Init message received");
                e.ports[0].onmessage = onPortMessge;
                port = e.ports[0];
            }
            window.addEventListener("message", onMessage);
            "#
        </Script>
        <NotificationPopupComponent/>
        <Router fallback=|| {
            view! {
                // let mut outside_errors = Errors::default();
                // outside_errors.insert_with_default_key(AppError::NotFound);
                <p>Error</p>
            }
                .into_view()
        }>
            <main id="content" class="h-screen bg-gray-900">

                <Routes>
                    <Route
                        path="/extension"
                        view=move || {
                            view! { <Outlet/> }
                        }
                    >

                        <Route path="/" view=WebServerExtensionHomePage/>
                    </Route>
                    <Route
                        path="/ui"
                        view=move || {
                            view! {
                                // only show the outlet if data have loaded
                                // <Show when=|| is_loaded() fallback=|| view! { <p>"Loading"</p> }>
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
                </Routes>
            </main>
        </Router>
    }
}
