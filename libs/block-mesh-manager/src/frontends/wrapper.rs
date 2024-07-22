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
    let resources_ready = Signal::derive(move || {
        let r = resource.is_some_and(|r| r.get().is_some());
        let a = auth.is_some_and(|a| a.get().is_some());
        r || a
    });
    view! {
        <Suspense fallback=move || { load.with_value(|v| v.clone().into_view()) }>
            <Show fallback=move || view! {} when=move || resources_ready.get()>
                {move || view.get()}
            </Show>
        </Suspense>
    }
}
