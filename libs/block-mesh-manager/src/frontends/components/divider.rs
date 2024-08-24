use leptos::*;
use tailwind_fuse::*;

#[component]
pub fn Divider(class: &'static str) -> impl IntoView {
    let class = tw_join!(class, "w-full border-t",);

    view! { <hr role="presentation" class=class /> }
}
