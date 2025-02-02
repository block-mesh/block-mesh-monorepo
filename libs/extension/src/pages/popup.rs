use crate::components::wrapper::Wrapper;
use crate::pages::popup_pages::loading::ExtensionLoading;
use crate::pages::popup_pages::logged_in::ExtensionLoggedIn;
use crate::pages::popup_pages::login::ExtensionLogin;
use crate::pages::popup_pages::register::ExtensionRegister;
use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use leptos::*;

#[component]
pub fn ExtensionPopupPage() -> impl IntoView {
    provide_context(ExtensionWrapperState::default());
    let state = use_context::<ExtensionWrapperState>().unwrap();
    let _state = ExtensionWrapperState::init_resource(state);

    view! {
        // <Wrapper
        //     resource=none_extension_resource
        //     loading=|| view! { Loading Resource }
        //     class=""
        // >
            <ExtensionRegister/>
        // </Wrapper>
        // <ExtensionLoading/>
        // <ExtensionLoggedIn/>
        // <ExtensionLogin/>

    }
}
