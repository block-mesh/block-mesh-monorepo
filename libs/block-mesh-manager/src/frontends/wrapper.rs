use leptos::*;

#[component]
pub fn Wrapper<S1, T1, S2, T2>(
    children: ChildrenFn,
    resource: Option<Resource<S1, T1>>,
    auth: Option<Resource<S2, T2>>,
    loading: impl IntoView + 'static + Clone,
) -> impl IntoView
where
    S1: 'static + Clone,
    T1: 'static + Clone,
    S2: 'static + Clone,
    T2: 'static + Clone,
{
    let (view, _) = create_signal(move || children().into_view());
    let load = store_value(loading);
    let resources_ready = Signal::derive(move || match (resource, auth) {
        (None, None) => true,
        (Some(r), None) => r.get().is_some(),
        (None, Some(a)) => a.get().is_some(),
        (Some(r), Some(a)) => r.get().is_some() && a.get().is_some(),
    });
    view! {
        <Suspense fallback=move || { load.with_value(|v| v.clone().into_view()) }>
            <Show fallback=move || view! {} when=move || resources_ready.get()>
                {move || view.get()}
            </Show>
        </Suspense>
    }
}
