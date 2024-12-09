use crate::routes;
use crate::startup::application::AppState;
use axum::routing::{get, post};
use axum::Router;
use block_mesh_common::routes_enum::RoutesEnum;
use std::sync::Arc;

pub fn get_api_router() -> Router<Arc<AppState>> {
    let api_router = Router::new()
        .route(
            RoutesEnum::Api_ApplyRanking.to_string().as_str(),
            post(routes::perks::apply_ranking::handler),
        )
        .route(
            RoutesEnum::Api_ConnectWallet.to_string().as_str(),
            post(routes::perks::connect_wallet::handler),
        )
        .route(
            RoutesEnum::Api_ReportUptime.to_string().as_str(),
            post(routes::uptime_report::report_uptime::handler),
        )
        .route(
            RoutesEnum::Api_SubmitBandwidth.to_string().as_str(),
            post(routes::bandwidth::submit_bandwidth::handler),
        )
        .route(
            RoutesEnum::Api_GetToken.to_string().as_str(),
            post(routes::api_token::get_token::handler),
        )
        .route(
            RoutesEnum::Api_GetTask.to_string().as_str(),
            post(routes::tasks::get_task::handler),
        )
        .route(
            RoutesEnum::Api_SubmitTask.to_string().as_str(),
            post(routes::tasks::submit_task::handler),
        )
        .route(
            RoutesEnum::Api_GetStats.to_string().as_str(),
            post(routes::api_token::get_stats::handler),
        )
        .route(
            RoutesEnum::Api_GetLatestInviteCode.to_string().as_str(),
            post(routes::invite_codes::get_latest_invite_code::handler),
        )
        .route(
            RoutesEnum::Api_CheckToken.to_string().as_str(),
            post(routes::api_token::check_token::handler),
        )
        .route(
            RoutesEnum::Api_Dashboard.to_string().as_str(),
            post(routes::dashboard::dashboard_api::handler),
        )
        .route(
            RoutesEnum::Api_EMailViaToken.to_string().as_str(),
            post(routes::api_token::get_email_via_token::handler),
        )
        .route(
            RoutesEnum::Api_ReportsQueue.to_string().as_str(),
            get(routes::admin::reports_queue::get_stats::handler)
                .post(routes::admin::reports_queue::change_settings::handler),
        );
    api_router
}
