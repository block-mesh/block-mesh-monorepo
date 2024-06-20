use crate::frontend::pages::home_page::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub fn app_css() -> &'static str {
    include_str!("./app.css")
}

#[component]
pub fn App() -> impl IntoView {
    let css = app_css();
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/poe.css"/>
        <Link href="https://fonts.googleapis.com/css2?family=Agbalumo&family=Varela+Round&family=Playfair+Display:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,500;1,600;1,700;1,800;1,900&display=swap" rel="stylesheet"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link href="https://fonts.googleapis.com/css2?family=Varela+Round&display=swap" rel="stylesheet"/>
        <Style>{css}</Style>
        <Title text="ImuDoc"/>
        <Router fallback=|| {
            // let mut outside_errors = Errors::default();
            // outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                Error
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
