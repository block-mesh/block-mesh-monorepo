use block_mesh_common::interfaces::server_api::DashboardResponse;
use leptos::*;

pub struct WebAppContext {
    pub points: RwSignal<f64>,
    pub number_of_users_invited: RwSignal<i64>,
}

impl WebAppContext {
    pub fn get_dashboard_data() -> Resource<Option<String>, Option<DashboardResponse>> {
        let (origin, set_origin) = create_signal(None::<String>);
        create_effect(move |_| {
            set_origin.set(Some(window().origin()));
        });

        create_resource(
            move || origin.get(),
            move |_| async move {
                origin.get()?;
                let client = reqwest::Client::new();
                let response = client
                    .post(&format!("{}/dashboard", origin.get().unwrap_or_default()))
                    .send()
                    .await;
                match response {
                    Ok(response) => match response.json::<DashboardResponse>().await {
                        Ok(json) => Some(json),
                        Err(e) => {
                            logging::log!("error: {}", e);
                            None
                        }
                    },
                    Err(e) => {
                        logging::log!("error: {}", e);
                        None
                    }
                }
            },
        )
    }
}
