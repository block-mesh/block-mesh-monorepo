use leptos::*;

#[component]
pub fn Wrapper<S, T>(
    children: ChildrenFn,
    resource: Resource<S, T>,
    loading: impl IntoView + 'static + Clone,
) -> impl IntoView
where
    S: 'static + Clone,
    T: 'static + Clone,
{
    let (view, _) = create_signal(move || children().into_view());
    let _load = store_value(loading);
    view! {
        <Suspense fallback=move || {
            // load.with_value(|v| v.clone().into_view())
            view! {  }
        }>{move || { resource.get().map(|_| view.get()) }}</Suspense>
    }
}
