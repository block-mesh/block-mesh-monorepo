use crate::routes::check_token::check_token;
use crate::routes::get_token::get_token;
use crate::routes::health::health;
use crate::routes::ok::ok_handler;
use crate::routes::version::version;
use axum::routing::{get, post};
use axum::Router;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/version", get(version))
        .route(
            "/api/check_token",
            post(check_token).get(ok_handler).options(ok_handler),
        )
        .route(
            "/api/get_token",
            post(get_token).get(ok_handler).options(ok_handler),
        )
}
