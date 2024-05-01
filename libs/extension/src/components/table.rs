use block_mesh_common::interface::Stat;
use leptos::*;

#[component]
pub fn Table(stats: ReadSignal<Vec<Stat>>) -> impl IntoView {
    view! {
        <table class="min-w-full divide-y divide-gray-100">
            <thead class="divide-x divide-gray-100 text-center">
                <tr class="divide-x divide-gray-100 text-center text-white">
                    <th>Date</th>
                    <th>Tasks Count</th>
                </tr>
            </thead>
            <tbody class="divide-y divide-gray-100">
                <For
                    each=move || stats.get()
                    key=move |stat| stat.day.to_string()
                    children=move |stat: Stat| {
                        view! {
                            <tr class="divide-x divide-gray-100 text-center text-white">
                                <td>{stat.day.to_string()}</td>
                                <td>{stat.tasks_count}</td>
                            </tr>
                        }
                    }
                />

            </tbody>
        </table>
    }
}
