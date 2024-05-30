use crate::components::table::Table;
use crate::utils::ext_state::{AppState, AppStatus};
use crate::utils::stats::get_stats;
use block_mesh_common::interfaces::server_api::{GetStatsRequest, Stat};
use leptos::*;
use std::time::Duration;

#[component]
pub fn Home() -> impl IntoView {
    let state = use_context::<AppState>().unwrap();
    let uptime = Signal::derive(move || state.uptime.get());
    let (stats, set_stats) = create_signal::<Vec<Stat>>(vec![]);

    let _interval = set_interval_with_handle(
        move || {
            if state.status.get() == AppStatus::LoggedIn {
                let email = state.email.get();
                let api_token = state.api_token.get();
                let blockmesh_url = state.blockmesh_url.get();
                spawn_local(async move {
                    if let Ok(result) =
                        get_stats(&blockmesh_url, &GetStatsRequest { email, api_token }).await
                    {
                        set_stats.set(result.stats);
                    }
                });
            }
        },
        Duration::from_secs(5),
    );

    view! {
        {move || match state.status.get() {
            AppStatus::LoggedIn => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center">
                        <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                            <p class="text-white mb-2">
                                {format!("Your uptime: {}", uptime.get())}
                            </p>
                            <Table stats=stats/>
                        </div>
                    </div>
                }
                    .into_view()
            }
            AppStatus::LoggedOut => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center">
                        <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                            <p class="text-white">You are not logged in</p>
                        </div>
                    </div>
                }
                    .into_view()
            }
            AppStatus::WaitingEmailVerification => {
                view! {
                    <div class="bg-gray-700 flex justify-center items-center">
                        <div class="bg-gray-800 border-white border-solid border-2 p-8 rounded-lg shadow-md w-80">
                            <p class="text-white">
                                Please verify your email address and login
                            </p>
                        </div>
                    </div>
                }
                    .into_view()
            }
        }}
    }
}
