use crate::proxy_pool::ProxyPool;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty};
use hyper::service::service_fn;
use hyper::{client, server, Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;

#[tracing::instrument(name = "listen_for_clients_connecting")]
pub async fn listen_for_clients_connecting(pool: ProxyPool, client_listener: TcpListener) {
    while let Ok((stream, addr)) = client_listener.accept().await {
        let pool = pool.clone();

        tokio::spawn(async move {
            if let Err(err) = server::conn::http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |req| handle_request(pool.clone(), req)),
                )
                .with_upgrades()
                .await
            {
                tracing::error!("Failed to serve connection from addr {:?}: {:?}", addr, err);
            }
        });
    }
}

#[tracing::instrument(name = "handle_request", ret, err)]
pub async fn handle_request(
    pool: ProxyPool,
    mut req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    if Method::CONNECT == req.method() {
        tracing::info!("CONNECT request from client");

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
            match hyper::upgrade::on(&mut req).await {
                Ok(upgraded) => {
                    let proxy = pool.get().await.unwrap();
                    let (mut send_request, conn) =
                        client::conn::http1::Builder::new().handshake(proxy).await?;

                    tokio::spawn(conn.with_upgrades());
                    let res = send_request.send_request(req).await?;

                    let stream = hyper::upgrade::on(res).await?;

                    copy_bidirectional(&mut TokioIo::new(upgraded), &mut TokioIo::new(stream))
                        .await
                        .unwrap();
                }
                Err(e) => eprintln!("upgrade error: {}", e),
            }
            Ok::<(), hyper::Error>(())
        });
        Ok(Response::new(empty()))
    } else {
        tracing::info!("NOT CONNECT request from client");

        let proxy = pool.get().await.unwrap();
        let (mut send_request, conn) = client::conn::http1::Builder::new().handshake(proxy).await?;

        tokio::spawn(conn);

        Ok(send_request.send_request(req).await?.map(|b| b.boxed()))
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
