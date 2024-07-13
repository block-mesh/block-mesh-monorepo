use crate::frontends::frontend_extension::extension_state::ExtensionState;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn ExtensionWrapper(children: ChildrenFn) -> impl IntoView {
    provide_context(ExtensionState::default());
    log!("ExtensionWrapper");
    let state = use_context::<ExtensionState>().unwrap();
    let resource = ExtensionState::init_resource(state);
    let (view, _) = create_signal(move || children().into_view());

    view! {
        <Suspense fallback=move || {
            view! { Loading... }
        }>{move || { resource.get().map(|_| view.get()) }}</Suspense>
    }
}
