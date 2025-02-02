use leptos::*;

#[component]
pub fn Wrapper<S1, T1>(
    children: ChildrenFn,
    resource: Option<Resource<S1, T1>>,
    loading: impl IntoView + 'static + Clone,
    class: &'static str,
) -> impl IntoView
where
    S1: 'static + Clone,
    T1: 'static + Clone,
{
    let (view, _) = create_signal(move || children().into_view());
    let load = store_value(loading);
    let resources_ready = Signal::derive(move || match (resource) {
        None => false,
        Some(r) => r.get().is_some(),
    });
    view! {
        <Suspense fallback=move || { load.with_value(|v| v.clone().into_view()) }>
            <Show fallback=move || view! {} when=move || resources_ready.get()>
                <div class=class>{move || view.get()}</div>
            </Show>
        </Suspense>
    }
}
