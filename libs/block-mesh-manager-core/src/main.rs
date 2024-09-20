#![allow(unused)]

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

    let app_state = AppState::new();
    let router = Router::new().with_state(app_state);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

struct AppState {
    pool: PgPool,
    environment: Environment,
}

impl AppState {
    async fn new() -> Self {
        let environment = std::env::var("APP_ENVIRONMENT").unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let pg_url = std::env::var("DATABASE_URL").unwrap();
        let pg_options = PgConnectOptions::from_str(&pg_url).unwrap();
        let pool = PgPool::connect_with(pg_options).await.unwrap();

        Self { pool, environment }
    }
}
