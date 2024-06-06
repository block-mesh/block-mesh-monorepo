use crate::components::navbar::NavBar;
use crate::components::notifications::Notifications;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page::Page;
use crate::pages::register::Register;
use crate::utils::ext_state::AppState;
use crate::utils::log::log;
use leptos::*;
use leptos_router::{use_navigate, Route, Router, Routes};

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    AppState::init_resource(state);
    create_effect(move |_| {
        let navigate = use_navigate();
        navigate(Page::Home.path(), Default::default());
    });

    let logout = create_action(move |_: &()| async move {
        state.clear().await;
    });

    let on_logout = move |_| {
        log!("Logout");
        logout.dispatch(());
    };

    view! {
        <Notifications/>
        <Router>
            <NavBar on_logout/>

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
