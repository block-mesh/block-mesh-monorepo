use leptos::*;

use crate::components::navbar::NavBar;
use crate::components::notifications::Notifications;
use crate::ext_router::ext_routes::ExtRoutes;
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

    let on_logout = move |_: ()| {
        log!("Logout");
        logout.dispatch(());
    };

    view! {
        <Notifications/>
        <NavBar on_logout/>
        <ExtRoutes/>
    }
}
