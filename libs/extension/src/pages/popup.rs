use leptos::*;

use crate::pages::test_page2::TestPage2;
use crate::utils::ext_state::AppState;

#[component]
pub fn Popup() -> impl IntoView {
    provide_context(AppState::default());
    let state = use_context::<AppState>().unwrap();
    let state = AppState::init_resource(state);

    let _logout = create_action(move |_: &()| async move {
        match state.get() {
            None => (),
            Some(s) => s.clear().await,
        };
    });

    // let on_logout = move |_| {
    //     log!("Logout");
    //     logout.dispatch(());
    // };

    view! {
        <TestPage2/>
    }
}
