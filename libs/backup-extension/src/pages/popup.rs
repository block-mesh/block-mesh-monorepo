use crate::components::navigator::ExtensionNavigator;
use crate::components::notifications::ExtensionWrapperNotifications;
use crate::pages::popup_pages::loading::ExtensionLoading;
use crate::pages::popup_pages::logged_in::ExtensionLoggedIn;
use crate::pages::popup_pages::login::ExtensionLogin;
use crate::pages::popup_pages::register::ExtensionRegister;
use crate::utils::extension_wrapper_state::{ExtensionWrapperState, Page};
use leptos::*;

#[component]
pub fn ExtensionPopupPage() -> impl IntoView {
    provide_context(ExtensionWrapperState::default());
    let state = use_context::<ExtensionWrapperState>().unwrap();
    let _state = ExtensionWrapperState::init_resource(state);

    view! {
        <ExtensionNavigator/>
        <ExtensionWrapperNotifications/>
        <Show
            when=move || state.page.get() == Page::Loading
            fallback=move || { view! {} }
        >
            <ExtensionLoading/>
        </Show>
        <Show
            when=move || state.page.get() == Page::LoggedIn
            fallback=move || { view! {} }
        >
            <ExtensionLoggedIn/>
        </Show>
        <Show
            when=move || state.page.get() == Page::Login
            fallback=move || { view! {} }
        >
            <ExtensionLogin/>
        </Show>
        <Show
            when=move || state.page.get() == Page::Register
            fallback=move || { view! {} }
        >
            <ExtensionRegister/>
        </Show>
    }
}
