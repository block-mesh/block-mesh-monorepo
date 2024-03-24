use crate::clients_endpoint::listen_for_clients_connecting;
use crate::proxy_endpoint::listen_for_proxies_connecting;
use block_mesh_common::tracing::setup_tracing;
use futures_util::future::join_all;
use proxy_pool::ProxyPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod clients_endpoint;
mod proxy_endpoint;
mod proxy_pool;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();
    let pool = ProxyPool::default();
    let addr_proxies = SocketAddr::from(([127, 0, 0, 1], 5000));
    let proxy_listener = TcpListener::bind(addr_proxies).await?;
    tracing::info!("Listening on for proxies on: {}", addr_proxies);
    let addr_clients = SocketAddr::from(([127, 0, 0, 1], 4000));
    let client_listener = TcpListener::bind(addr_clients).await?;
    tracing::info!("Listening on for clients on: {}", addr_clients);

    let proxy_listener_pool = pool.clone();
    let proxy_listener_task = tokio::task::spawn(async move {
        listen_for_proxies_connecting(proxy_listener_pool, proxy_listener).await
    });
    let proxy_listener_pool = pool.clone();
    let clients_listener_task = tokio::task::spawn(async move {
        listen_for_clients_connecting(proxy_listener_pool, client_listener).await;
    });
    let _ = join_all(vec![proxy_listener_task, clients_listener_task]).await;
    Ok(())
}
