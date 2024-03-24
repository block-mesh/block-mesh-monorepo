use bytes::Bytes;
use futures_util::future::join;
use http_body_util::{combinators::BoxBody, BodyExt, Empty};
use hyper::{server, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use proxy_pool::ProxyPool;
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
            let pool = pool1.clone();
            if let Err(err) = server::conn::http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |req| proxy_endpoint(pool.clone(), req)),
                )
                .with_upgrades()
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
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
    });

    let _ret = join(task_1, task_2).await;
    Ok(())
}

async fn proxy_endpoint(
    pool: ProxyPool,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    println!("req: {:?}", req);

    // Received an HTTP request like:
    // ```
    // CONNECT www.domain.com:443 HTTP/1.1
    // Host: www.domain.com:443
    // Proxy-Connection: Keep-Alive
    // ```
    //
    // When HTTP method is CONNECT we should return an empty body
    // then we can eventually upgrade the connection and talk a new protocol.
    //
    // Note: only after client received an empty body with STATUS_OK can the
    // connection be upgraded, so we can't return a response inside
    // `on_upgrade` future.
    tokio::spawn(async move {
        match hyper::upgrade::on(req).await {
            Ok(upgraded) => {
                // We can put proxy along with req here
                pool.put(upgraded).await;
            }
            Err(e) => eprintln!("upgrade error: {}", e),
        }
    });

    Ok(Response::new(empty()))
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
