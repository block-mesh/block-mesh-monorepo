use crate::routes::check_token::check_token;
use crate::routes::get_token::get_token;
use axum::routing::post;
use axum::Router;

pub fn get_router() -> Router {
    Router::new()
        .route("/api/check_token", post(check_token))
        .route("/api/get_token", post(get_token))
}
