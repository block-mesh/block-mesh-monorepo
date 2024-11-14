use crate::routes::check_token::check_token;
use crate::routes::get_token::get_token;
use crate::routes::health::{db_health, server_health};
use crate::routes::ok::ok_handler;
use crate::routes::version::version;
use axum::routing::{get, post};
use axum::Router;
use block_mesh_common::constants::DeviceType;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(server_health))
        .route("/server_health", get(server_health))
        .route("/db_health", get(db_health))
        .route("/version", get(version))
        .route(
            "/api/check_token",
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            "/api/get_token",
            post(get_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/check_token", DeviceType::Extension),
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/get_token", DeviceType::Extension),
            post(get_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/check_token", DeviceType::Cli),
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/get_token", DeviceType::Cli),
            post(get_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/check_token", DeviceType::AppServer),
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/get_token", DeviceType::AppServer),
            post(get_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/check_token", DeviceType::Worker),
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/get_token", DeviceType::Worker),
            post(get_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/check_token", DeviceType::Unknown),
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            &format!("/{}/api/get_token", DeviceType::Unknown),
            post(get_token).get(ok_handler).options(ok_handler),
        )
}
