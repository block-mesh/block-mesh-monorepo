mod background;
mod components;
mod pages;
mod utils;
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::components::navbar::NavBar;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::page::Page;
use crate::pages::register::Register;
use crate::utils::connectors::set_panic_hook;
use crate::utils::state::AppState;
#[allow(unused_imports)]
use background::*;
#[allow(unused_imports)]
use pages::*;

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());
    spawn_local(async move {
        let state1 = expect_context::<AppState>();
        let state2 = AppState::new().await;
        if let Ok(state) = state2 {
            state1
                .blockmesh_url
                .update(|v| *v = state.blockmesh_url.get_untracked());
            state1.email.update(|v| *v = state.email.get_untracked());
            state1
                .api_token
                .update(|v| *v = state.api_token.get_untracked());
        };
    });

    let logout = create_action(move |_: &()| async move {});

    let on_logout = move |_| {
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
                                    view! { <Home  /> }
                                }
                            />
                            <Route
                                path=Page::Login.path()
                                view=move || {
                                    view! {
                                        <Login />
                                    }
                                }
                            />
                            <Route
                                path=Page::Register.path()
                                view=move || {
                                    view! { <Register /> }
                                }
                            />
                    </Routes>
            </main>
        </Router>
    }
}

#[wasm_bindgen]
pub fn mount_popup() {
    set_panic_hook();
    mount_to_body(Popup);
}

#[component]
pub fn Options() -> impl IntoView {}

#[wasm_bindgen]
pub fn mount_options() {
    set_panic_hook();
    mount_to_body(Options);
}
