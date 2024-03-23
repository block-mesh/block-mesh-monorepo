use futures_util::future::join;
use std::fmt::Display;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

use crate::connect_streams_via_channel::connect_streams_via_channel;

mod connect_streams_via_channel;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    FromUpgraded,
    FromServer,
}

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Client,
    Proxy,
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::Client => write!(f, "client"),
            Context::Proxy => write!(f, "proxy"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr_clients = SocketAddr::from(([127, 0, 0, 1], 4000));
    let addr_proxies = SocketAddr::from(([127, 0, 0, 1], 5000));
    let (client_sender, client_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
        broadcast::channel(16);
    let (proxy_sender, proxy_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) =
        broadcast::channel(16);

    let client_listener = TcpListener::bind(addr_clients).await?;
    println!("Listening on http://{}", addr_clients);
    let proxy_listener = TcpListener::bind(addr_proxies).await?;
    println!("Listening on http://{}", addr_proxies);

    let task_1 = tokio::task::spawn(async move {
        connect_streams_via_channel(
            client_listener,
            client_sender,
            proxy_receiver,
            Context::Client,
        )
        .await
    });

    let task_2 = tokio::task::spawn(async move {
        connect_streams_via_channel(
            proxy_listener,
            proxy_sender,
            client_receiver,
            Context::Proxy,
        )
        .await
    });
    let _ret = join(task_1, task_2).await;
    Ok(())
}
