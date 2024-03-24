use crate::proxy_pool::ProxyPool;
use crate::utils::empty;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::server;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tracing::instrument(name = "listen_task")]
pub async fn listen_for_proxies_connecting(pool: ProxyPool, proxy_listener: TcpListener) -> () {
    while let Ok((stream, addr)) = proxy_listener.accept().await {
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

pub async fn handle_request(
    pool: ProxyPool,
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    tracing::info!("handle_request: {:?}", req);
    if Method::CONNECT == req.method() {
        tracing::info!("CONNECT request");

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
    } else {
        // TODO : Process request - can register proxy here
        tracing::info!("NOT CONNECT request");
        Ok(Response::new(empty()))
    }
}
