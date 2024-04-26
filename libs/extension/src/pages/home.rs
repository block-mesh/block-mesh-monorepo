use crate::pages::page::Page;
use crate::utils::state::AppState;
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let logged_in = Signal::derive(move || state.logged_in.get());
    let email = Signal::derive(move || state.email.get());

    view! {
        <h2>"Leptos Login example"</h2>
        {move || match logged_in.get() {
            true => {
                view! { <p>"You are logged in with " {email} "."</p> }.into_view()
            }
            false => {
                view! {
                    <p>"You are not logged in."</p>
                    <a href=Page::Login.path()>"Login now."</a>
                }.into_view()
            }
        }}
    }
}
