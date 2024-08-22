use block_mesh_common::interfaces::server_api::{
    AuthStatusResponse, DailyLeaderboard, DashboardResponse,
};
use block_mesh_common::routes_enum::RoutesEnum;
use leptos::*;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct WebAppContext {
    pub points: RwSignal<f64>,
    pub number_of_users_invited: RwSignal<i64>,
    pub wallet_address: RwSignal<String>,
}

impl Default for WebAppContext {
    fn default() -> Self {
        Self {
            points: create_rw_signal(0.0),
            number_of_users_invited: create_rw_signal(0),
            wallet_address: create_rw_signal(String::default()),
        }
    }
}

impl WebAppContext {
    pub fn get_daily_leaderboard() -> Resource<Option<String>, Option<DailyLeaderboard>> {
        let (origin, set_origin) = create_signal(None::<String>);
        create_effect(move |_| {
            set_origin.set(Some(window().origin()));
        });

        create_local_resource(
            move || origin.get(),
            move |_| async move {
                if let Some(origin) = origin.get_untracked() {
                    let client = reqwest::Client::new();
                    let response = client
                        .post(&format!(
                            "{}{}",
                            origin,
                            RoutesEnum::Static_Auth_Daily_Leaderboard
                        ))
                        .send()
                        .await;
                    match response {
                        Ok(response) => match response.json::<DailyLeaderboard>().await {
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
