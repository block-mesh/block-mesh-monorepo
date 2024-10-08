use crate::state::AppState;
use crate::websocket::ws_handler::ws_handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

pub async fn app(listener: TcpListener, state: AppState) {
    let router = Router::new()
        .route("/health", get(health))
        .route("/ws", get(ws_handler))
        .with_state(Arc::new(state));

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
