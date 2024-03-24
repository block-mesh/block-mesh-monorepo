use futures_util::future::join;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

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
            let pool = pool.clone();
            tokio::spawn(async move {
                let mut buf = Vec::new();

                loop {
                    let mut headers = [httparse::EMPTY_HEADER; 64];
                    let mut req = httparse::Request::new(&mut headers);

                    let n = stream.read_buf(&mut buf).await?;

                    if n == 0 {
                        return Ok(());
                    }

                    if let Ok(httparse::Status::Complete(len)) = req.parse(&buf) {
                        // We can pick proxy by req
                        let mut proxy = pool.get().await.unwrap();

                        // We can modify the request here
                        proxy.write_all(&buf[..len]).await?;

                        // Write the rest of the buffer
                        proxy.write_all(&buf[len..]).await?;
                        tokio::io::copy_bidirectional(&mut stream, &mut proxy).await?;
                        break;
                    }
                }

                Ok::<(), std::io::Error>(())
            });
        }
    });

    let _ret = join(task_1, task_2).await;
    Ok(())
}
