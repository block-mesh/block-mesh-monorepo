use crate::frontend::pages::dashboard_page::DashboardPage;
use crate::frontend::pages::login_page::LoginPage;
use crate::frontend::pages::new_password_page::NewPasswordPage;
use crate::frontend::pages::register_page::RegisterPage;
use crate::frontend::pages::resend_confirmation_email_page::ResendConfirmationEmailPage;
use crate::frontend::pages::reset_password_page::ResetPasswordPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
        <Router fallback=|| {
            view! {
                // let mut outside_errors = Errors::default();
                // outside_errors.insert_with_default_key(AppError::NotFound);
                Error
            }
                .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/reset_password" view=ResetPasswordPage/>
                    <Route path="/register" view=RegisterPage/>
                    <Route path="/dashboard" view=DashboardPage/>
                    <Route path="/resend_confirmation_email" view=ResendConfirmationEmailPage/>
                    <Route path="/new_password" view=NewPasswordPage/>
                // <ProtectedRoute path="profile" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
