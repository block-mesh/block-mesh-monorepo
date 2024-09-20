#![allow(unused)]

pub mod errors;
pub mod state;
pub mod websocket;

use crate::state::AppState;
use crate::websocket::manager::WebSocketManager;
use crate::websocket::ws_handler::ws_handler;
use axum::extract::State;
use axum::routing::get;
use axum::{Router, ServiceExt};
use block_mesh_common::env::environment::Environment;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening at {addr}");

    let app_state = AppState::new().await;
    let router = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
