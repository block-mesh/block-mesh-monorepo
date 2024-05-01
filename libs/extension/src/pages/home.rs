use crate::pages::page::Page;
use crate::utils::state::{AppState, AppStatus};
use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let status = Signal::derive(move || state.status.get());
    let email = Signal::derive(move || state.email.get());

    // create_effect(move |_| {
    //     log!("new effect");
    //     spawn_local(async move {
    //         log!("new spawn");
    //         while let status = state.status.get() {
    //             log!("Spinning around {:#?}", state);
    //             sleep(Duration::from_secs(5)).await;
    //         }
    //     });
    // });

    view! {
        <h2>"BlockMesh Network"</h2>
        {move || match status.get() {
            AppStatus::LoggedIn => {
                view! { <p>"You are logged in with " {email} "."</p> }.into_view()
            }
            AppStatus::LoggedOut => {
                view! {
                    <p>"You are not logged in."</p>
                    <a href=Page::Login.path()>"Login now."</a>
                }
                    .into_view()
            }
            AppStatus::WaitingEmailVerification => {
                view! {
                    <p>"You are logged in, but your email is not verified yet."</p>
                    <a href=Page::Login.path()>"Login now."</a>
                }
                    .into_view()
            }
        }}
    }
}
