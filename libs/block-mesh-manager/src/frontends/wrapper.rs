use leptos::*;

#[component]
pub fn Wrapper<S, T>(children: ChildrenFn, resource: Resource<S, T>) -> impl IntoView
where
    S: 'static + Clone,
    T: 'static + Clone,
{
    let (view, _) = create_signal(move || children().into_view());
    view! {
        <Suspense fallback=move || {
            view! { Loading... }
        }>{move || { resource.get().map(|_| view.get()) }}</Suspense>
    }
}
