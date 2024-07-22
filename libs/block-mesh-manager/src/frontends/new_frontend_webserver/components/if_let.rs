use leptos::*;
use std::rc::Rc;

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
            opt
                .with(|value| {
                    view
                        .with_value(|view| {
                            value
                                .as_ref()
                                .map_or_else(|| fallback.run(), |value| view(value).into_view())
                        })
                })
        }}
    }
}

#[derive(Clone)]
pub struct ErrViewFn<E>(Rc<dyn Fn(&E) -> View>);

impl<E> Default for ErrViewFn<E> {
    fn default() -> Self {
        Self(Rc::new(|_| ().into_view()))
    }
}

impl<F, IV, E> From<F> for ErrViewFn<E>
where
    F: Fn(&E) -> IV + 'static,
    IV: IntoView,
{
    fn from(value: F) -> Self {
        Self(Rc::new(move |err| value(err).into_view()))
    }
}

impl<E> ErrViewFn<E> {
    /// Execute the wrapped function
    pub fn run(&self, err: &E) -> View {
        (self.0)(err)
    }
}

#[component]
pub fn IfLetOk<T, E, CF, V>(
    #[prop(into)] opt: MaybeSignal<Result<T, E>>,
    #[prop(optional, into)] fallback: ErrViewFn<E>,
    children: CF,
) -> impl IntoView
where
    T: 'static,
    E: 'static,
    CF: Fn(&T) -> V + 'static,
    V: IntoView + 'static,
{
    let view = store_value(children);

    view! {
        {move || {
            opt
                .with(|value| {
                    view
                        .with_value(|view| {
                            value
                                .as_ref()
                                .map_or_else(
                                    |err| fallback.run(err),
                                    |value| view(value).into_view(),
                                )
                        })
                })
        }}
    }
}
