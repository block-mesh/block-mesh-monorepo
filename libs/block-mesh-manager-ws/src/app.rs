use crate::state::AppState;
use crate::websocket::ws_handler::ws_handler;
use axum::routing::get;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use tokio::net::TcpListener;

pub async fn app(listener: TcpListener, state: AppState) {
    let router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
