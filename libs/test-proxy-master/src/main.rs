use futures_util::future::join;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod proxy_pool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = proxy_pool::ProxyPool::default();

    let addr_clients = SocketAddr::from(([127, 0, 0, 1], 4000));
    let addr_proxies = SocketAddr::from(([127, 0, 0, 1], 5000));

    let client_listener = TcpListener::bind(addr_clients).await?;
    println!("Listening on http://{}", addr_clients);
    let proxy_listener = TcpListener::bind(addr_proxies).await?;
    println!("Listening on http://{}", addr_proxies);

    let pool1 = pool.clone();
    let task_1 = tokio::task::spawn(async move {
        while let Ok((stream, _addr)) = proxy_listener.accept().await {
            pool1.put(stream).await;
        }
    });

    let task_2 = tokio::task::spawn(async move {
        while let Ok((mut stream, _addr)) = client_listener.accept().await {
            // Simple round-robin
            let mut proxy = pool.get().await.unwrap();
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
