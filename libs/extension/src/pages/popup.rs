use leptos::*;

use crate::utils::ext_state::AppState;
use crate::utils::log::log;

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    let state = AppState::init_resource(state);

    let logout = create_action(move |_: &()| async move {
        match state.get() {
            None => (),
            Some(s) => s.clear().await,
        };
    });

    let _on_logout = move |_: ()| {
        log!("Logout");
        logout.dispatch(());
    };

    view! {
        // <iframe width="300" height="400" src="http://localhost:8000/ext/login"/>
        // <div class="bg-image bg-fixed bg-cover bg-no-repeat bg-gray-800">
        //     <Router>
        //         <Notifications/>
        //         <NavBar on_logout/>
        //
        //         <main>
        //             <Routes>
        //                 <Route
        //                     path=Page::Home.path()
        //                     view=move || {
        //                         view! { <Home/> }
        //                     }
        //                 />
        //
        //                 <Route
        //                     path=Page::Login.path()
        //                     view=move || {
        //                         view! {
        //                             <Login on_success=move |_: ()| {
        //                                 let navigate = use_navigate();
        //                                 navigate(Page::Home.path(), Default::default());
        //                             }/>
        //                         }
        //                     }
        //                 />
        //
        //                 <Route
        //                     path=Page::Register.path()
        //                     view=move || {
        //                         view! {
        //                             <Register on_success=move |_: ()| {
        //                                 let navigate = use_navigate();
        //                                 navigate(Page::Home.path(), Default::default());
        //                             }/>
        //                         }
        //                     }
        //                 />
        //
        //             </Routes>
        //         </main>
        //     </Router>
        // </div>
    }
}
