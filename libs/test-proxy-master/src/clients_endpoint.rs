use crate::proxy_pool::ProxyPool;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn listen_for_clients_connecting(pool: ProxyPool, client_listener: TcpListener) {
    while let Ok((mut stream, _addr)) = client_listener.accept().await {
        let pool = pool.clone();
        tokio::spawn(async move {
            let mut buf: Vec<u8> = Vec::new();

            loop {
                let mut headers = [httparse::EMPTY_HEADER; 64];
                let mut req = httparse::Request::new(&mut headers);
                let n = stream.read_buf(&mut buf).await?;
                if n == 0 {
                    panic!("invalid request")
                }
                if let Ok(httparse::Status::Complete(len)) = req.parse(&buf) {
                    // We can pick proxy by req
                    let mut proxy = TokioIo::new(pool.get().await.unwrap());

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
}
