use leptos::*;
use leptos_router::{use_navigate, Route, Router, Routes};

use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page::Page;
use crate::pages::register::Register;

#[component]
pub fn ExtRoutes() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route
                        path=Page::Home.path()
                        view=move || {
                            view! { <Home/> }
                        }
                    />

                    <Route
                        path=Page::Login.path()
                        view=move || {
                            view! {
                                <Login on_success=move |_: ()| {
                                    let navigate = use_navigate();
                                    navigate(Page::Home.path(), Default::default());
                                }/>
                            }
                        }
                    />

                    <Route
                        path=Page::Register.path()
                        view=move || {
                            view! {
                                <Register on_success=move |_: ()| {
                                    let navigate = use_navigate();
                                    navigate(Page::Home.path(), Default::default());
                                }/>
                            }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
