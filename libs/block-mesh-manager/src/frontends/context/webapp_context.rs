use block_mesh_common::interfaces::server_api::{AuthStatusResponse, DashboardResponse};
use leptos::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct WebAppContext {
    pub points: RwSignal<f64>,
    pub number_of_users_invited: RwSignal<i64>,
}

impl Default for WebAppContext {
    fn default() -> Self {
        Self {
            points: create_rw_signal(0.0),
            number_of_users_invited: create_rw_signal(0),
        }
    }
}

impl WebAppContext {
    pub fn get_dashboard_data() -> Resource<Option<String>, Option<DashboardResponse>> {
        let (origin, set_origin) = create_signal(None::<String>);
        create_effect(move |_| {
            set_origin.set(Some(window().origin()));
        });

        create_local_resource(
            move || origin.get(),
            move |_| async move {
                if let Some(origin) = origin.get_untracked() {
                    let client = reqwest::Client::new();
                    let response = client.post(&format!("{}/dashboard", origin)).send().await;
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
                } else {
                    None
                }
            },
        )
    }

    pub fn is_logged_in() -> Resource<Option<String>, Option<AuthStatusResponse>> {
        let (origin, set_origin) = create_signal(None::<String>);
        create_effect(move |_| {
            set_origin.set(Some(window().origin()));
        });

        create_local_resource(
            move || origin.get(),
            move |_| async move {
                if let Some(origin) = origin.get_untracked() {
                    let client = reqwest::Client::new();
                    let response = client.get(&format!("{}/auth_status", origin)).send().await;
                    match response {
                        Ok(response) => match response.json::<AuthStatusResponse>().await {
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
                } else {
                    None
                }
            },
        )
    }
}
