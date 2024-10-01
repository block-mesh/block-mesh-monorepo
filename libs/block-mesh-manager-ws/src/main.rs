// use axum::extract::State;
// use axum::routing::get;
// use axum::{Router, ServiceExt};
// use block_mesh_common::env::environment::Environment;
use block_mesh_manager_ws::app::app;
use block_mesh_manager_ws::state::AppState;
use dotenv::dotenv;
// use sqlx::postgres::PgConnectOptions;
// use sqlx::PgPool;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::process;
// use std::str::FromStr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening at {addr}");

    let state = AppState::new().await;
    app(listener, state).await;
    process::exit(1);
}
