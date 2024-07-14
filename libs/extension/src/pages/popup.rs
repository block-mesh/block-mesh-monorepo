use crate::utils::extension_wrapper_state::ExtensionWrapperState;
use leptos::*;

#[component]
pub fn ExtensionPopupPage() -> impl IntoView {
    provide_context(ExtensionWrapperState::default());
    let state = use_context::<ExtensionWrapperState>().unwrap();
    let _state = ExtensionWrapperState::init_resource(state);
    view! {}
}
