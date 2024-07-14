use crate::leptos_state::LeptosTauriAppState;
use block_mesh_common::app_config::TaskStatus;
use leptos::*;

#[component]
pub fn Dashboard(task_status: ReadSignal<String>) -> impl IntoView {
    let state = expect_context::<LeptosTauriAppState>();
    let mode = Signal::derive(move || state.app_config.get().mode.unwrap_or_default().to_string());
    let status_color = move || match state.app_config.get().task_status {
        None => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-red-600",
        Some(task_status) => match task_status {
            TaskStatus::Running => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-green-600",
            TaskStatus::Off => "mt-1 text-sm sm:mt-0 sm:col-span-2 text-red-600",
        },
    };

    view! {
        <div class="bg-gray-800 container mx-auto my-8">
            <div class="shadow-md rounded-lg overflow-hidden">
                <div class="px-4 py-5 sm:px-6 bg-gray-900">
                    <h2 class="text-xl leading-6 font-medium text-white">Current Status</h2>
                </div>
                <div class="border-t border-white">
                    <dl>
                        <div class="bg-gray-700 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                            <dt class="text-sm font-medium text-gray-500">{move || mode.get()}</dt>
                            <dd class=status_color>{move || task_status.get()}</dd>
                        </div>
                    </dl>
                </div>
            </div>
        </div>
    }
}
