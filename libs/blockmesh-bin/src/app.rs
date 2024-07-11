use std::time::Duration;

use leptos::*;

use crate::app_router::AppRouter;
use crate::components::app_footer::AppFooter;
use crate::leptos_state::LeptosTauriAppState;

#[component]
pub fn App() -> impl IntoView {
    provide_context(LeptosTauriAppState::default());
    let state = expect_context::<LeptosTauriAppState>();
    let resource = create_local_resource(
        move || {
            state.app_config.get().email.is_some() || state.app_config.get().api_token.is_some()
        },
        |_| async move {
            let state = expect_context::<LeptosTauriAppState>();
            LeptosTauriAppState::init_app_config(&state).await;
            LeptosTauriAppState::check_token(&state).await;
            LeptosTauriAppState::get_task_status(&state).await;
            LeptosTauriAppState::get_ore_status(&state).await;
        },
    );

    let _interval = set_interval_with_handle(
        move || {
            spawn_local(async move {
                let state = expect_context::<LeptosTauriAppState>();
                LeptosTauriAppState::get_task_status(&state).await;
                LeptosTauriAppState::get_ore_status(&state).await;
            });
        },
        Duration::from_secs(10),
    );

    view! {
        <div class="h-screen bg-gray-800">
            <Suspense fallback=move || view! { <p>Loading</p> }>
                <div class="hidden">{resource.get()}</div>
                <div class="h-full">
                    <AppRouter/>
                    <AppFooter/>
                </div>
            </Suspense>
        </div>
    }
}
