use crate::routes::check_token::check_token;
use axum::routing::post;
use axum::Router;

pub fn get_router() -> Router {
    Router::new().route("/api/check_token", post(check_token))
}
