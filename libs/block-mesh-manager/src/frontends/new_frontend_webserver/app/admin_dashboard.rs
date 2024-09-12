use crate::frontends::components::heading::Heading;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use leptos::logging::log;
use leptos::*;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    // FIXME wasm error
    let (stats, set_stats) = create_signal(String::from(""));
    let stats_resource = create_local_resource(
        move || (),
        move |_| async move {
            let client = Client::new();
            let response = client
                .get(format!("{}/api/admin/reports_queue", window().origin()))
                .send()
                .await;
            log!("19 response {:#?}", response);
            let json: Value = response.unwrap().json().await.unwrap();
            set_stats.set(format!(
                "Messages: {:?}\nWindow size: {:?}\nUsed window size: {:?}\nQueue size: {:?}\n Period: {:?}",
                json.get("messages"),
                json.get("window_size"),
                json.get("used_window_size"),
                json.get("queue_size"),
                json.get("period")
            ));
        },
    );

    let (window_size, windows_size_set) = create_signal(String::from(""));
    let (period, period_set) = create_signal(String::from(""));

    let update_settings = create_action(move |_input: &()| async move {
        let client = Client::new();
        let period: Option<u64> = period.get().parse().ok();
        let window_size: Option<usize> = window_size.get().parse().ok();
        let response = client
            .post(format!("{}/api/admin/reports_queue", window().origin(),))
            .json(&json!({
                "period": period.map(Duration::from_secs),
                "messages": None::<Vec<String>>,
                "window_size": window_size
            }))
            .send()
            .await;
        stats_resource.refetch();
        log!("19 response {:#?}", response);
    });
    view! {
        <ApplicationLayout>
            <div class="flex items-start justify-start gap-4">
                <Heading>Admin Dashboard</Heading>
            </div>
            <div class="flex flex-col justify-start gap-10">
            <label class="text-off-white">Window size</label>
            <input type="number" placeholder="Window size (queue batch size)" prop:value=window_size on:input=move |ev| {
                    windows_size_set.set(event_target_value(&ev));
            }/>
            <label class="text-off-white">Period (seconds)</label>
            <input type="number" placeholder="Period in seconds" prop:value=period on:input=move |ev| {
                period_set.set(event_target_value(&ev));
            }/>
            <button class="text-off-white" on:click=move |_| {update_settings.dispatch(())}>Update Settings</button>
            <button class="text-off-white" on:click=move |_| {stats_resource.refetch()}>Refresh Stats</button>
            </div>
            <pre class="text-off-white">
                {move || stats.get()}
            </pre>
        </ApplicationLayout>
    }
}
