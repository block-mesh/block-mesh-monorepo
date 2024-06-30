use crate::components::navigation::Navigation;
use crate::page_routes::PageRoutes;
use crate::pages::apps::ore_wrapper::OreWrapper;
use crate::pages::apps::select_app::SelectApps;
use crate::pages::config_viewer::ConfigViewer;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::register::Register;
use crate::pages::settings_wrapper::SettingsWrapper;
use leptos::*;
use leptos_router::{Route, Router, Routes};

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <Navigation/>
            <div class="lg:pl-72">
                <main>
                    <div class="px-4 sm:px-6 lg:px-8">
                        <Routes>
                            <Route
                                path=PageRoutes::Home.path()
                                view=move || {
                                    view! { <Home/> }
                                }
                            />

                            // <Route
                            // path=PageRoutes::Dashboard.path()
                            // view=move || {
                            // view! { <Dashboard task_status/> }
                            // }
                            // />

                            <Route
                                path=PageRoutes::Settings.path()
                                view=move || {
                                    view! { <SettingsWrapper/> }
                                }
                            />

                            <Route
                                path=PageRoutes::OreMiner.path()
                                view=move || {
                                    view! { <OreWrapper/> }
                                }
                            />

                            <Route
                                path=PageRoutes::Apps.path()
                                view=move || {
                                    view! { <SelectApps/> }
                                }
                            />

                            <Route
                                path=PageRoutes::ConfigViewer.path()
                                view=move || {
                                    view! { <ConfigViewer/> }
                                }
                            />

                            <Route
                                path=PageRoutes::Login.path()
                                view=move || {
                                    view! { <Login/> }
                                }
                            />

                            <Route
                                path=PageRoutes::Register.path()
                                view=move || {
                                    view! { <Register/> }
                                }
                            />

                        </Routes>
                    </div>
                </main>
            </div>
        </Router>
    }
}
