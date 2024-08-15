use crate::startup::application::AppState;
use crate::ws::ws_handler::ws_handler;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

pub fn get_ws_router() -> Router<Arc<AppState>> {
    let router = Router::new().route("/ws", get(ws_handler));
    router
}
