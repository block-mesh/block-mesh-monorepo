use crate::frontends::components::heading::Heading;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use leptos::logging::log;
use leptos::*;
use reqwest::Client;
use serde_json::Value;

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
                "{:?} {:?} {:?}",
                json.get("messages"),
                json.get("window_size"),
                json.get("used_window_size")
            ));
        },
    );

    let (window_size, windows_size_set) = create_signal(String::from(""));
    let (period, period_set) = create_signal(String::from(""));

    let update_settings = create_action(|input: &()| async {
        let client = Client::new();
        // let period: u64 = period.get().parse().unwrap();
        // let window_size: usize = window_size.get().parse().unwrap();
        let response = client
            .post(format!("{}/api/admin/reports_queue", window().origin(),))
            // .json(&json!({
            //     "period": Some(Duration::from_secs(1)),
            //     "messages": None::<Vec<String>>,
            //     "window_size": Some(10),
            // }))
            .send()
            .await;
        log!("19 response {:#?}", response);
    });
    view! {
        <ApplicationLayout>
            <div class="flex items-start justify-start gap-4">
                <Heading>Admin Dashboard</Heading>
            </div>
            <div class="flex flex-col justify-start gap-10">
            <input class="text-off-white" type="number" placeholder="Window size (queue batch size)" prop:value=window_size on:input=move |ev| {
                    windows_size_set.set(event_target_value(&ev));
            }/>
            <input type="number" placeholder="Period in seconds"/>
            <button class="text-off-white" on:click=move |_| {update_settings.dispatch(())}>Update Settings</button>
            <button class="text-off-white" on:click=move |_| {stats_resource.refetch()}>Refresh Stats</button>
            </div>
            <pre class="text-off-white">
                {stats}
            </pre>
        </ApplicationLayout>
    }
}
