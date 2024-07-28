use leptos::*;

#[component]
pub fn IfLetSome<T, CF, V>(
    #[prop(into)] opt: MaybeSignal<Option<T>>,
    #[prop(optional, into)] fallback: ViewFn,
    children: CF,
) -> impl IntoView
where
    T: 'static,
    CF: Fn(&T) -> V + 'static,
    V: IntoView + 'static,
{
    let view = store_value(children);

    view! {
        {move || {
            opt.with(|value| {
                view.with_value(|view| {
                    value.as_ref().map_or_else(|| fallback.run(), |value| view(value).into_view())
                })
            })
        }}
    }
}
