use futures_util::future::join;
use std::fmt::Display;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

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
    let (pool_sender, mut pool_receiver) = mpsc::unbounded_channel();

    let addr_clients = SocketAddr::from(([127, 0, 0, 1], 4000));
    let addr_proxies = SocketAddr::from(([127, 0, 0, 1], 5000));

    let client_listener = TcpListener::bind(addr_clients).await?;
    println!("Listening on http://{}", addr_clients);
    let proxy_listener = TcpListener::bind(addr_proxies).await?;
    println!("Listening on http://{}", addr_proxies);

    let task_1 = tokio::task::spawn(async move {
        while let Ok((stream, _addr)) = proxy_listener.accept().await {
            pool_sender.send(stream).unwrap();
        }
    });

    let task_2 = tokio::task::spawn(async move {
        while let Ok((mut stream, _addr)) = client_listener.accept().await {
            // Simple round-robin
            let mut proxy = pool_receiver.recv().await.unwrap();
            tokio::spawn(async move {
                tokio::io::copy_bidirectional(&mut stream, &mut proxy)
                    .await
                    .unwrap()
            });
        }
    });

    let _ret = join(task_1, task_2).await;
    Ok(())
}
