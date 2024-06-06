use crate::components::navbar::NavBar;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page::Page;
use crate::pages::register::Register;
use crate::utils::ext_state::AppState;
use crate::utils::log::log;
use lazy_static::lazy_static;
use leptos::*;
use leptos_router::{use_navigate, Route, Router, Routes};
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    static ref TEST_VARIABLE: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());

    log!("TEST_VARIABLE: {:?}", *TEST_VARIABLE.lock().unwrap());
    *TEST_VARIABLE.lock().unwrap() = true;
    let state = use_context::<AppState>().unwrap();
    AppState::init_resource(state);
    create_effect(move |_| {
        let navigate = use_navigate();
        navigate(Page::Home.path(), Default::default());
    });

    set_timeout(
        move || {
            log!(
                "timeout TEST_VARIABLE: {:?}",
                *TEST_VARIABLE.lock().unwrap()
            );
        },
        Duration::from_millis(1000),
    );

    let logout = create_action(move |_: &()| async move {
        state.clear().await;
    });

    let on_logout = move |_| {
        log!("Logout");
        logout.dispatch(());
    };

    view! {
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
