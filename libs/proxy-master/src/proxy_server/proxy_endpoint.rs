use crate::app_state::AppState;
use crate::proxy_server::proxy_pool::ProxyPool;
use crate::token_management::proxy_headers::process_proxy_headers;
use block_mesh_common::http::empty;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::server;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tracing::instrument(name = "listen_task", skip(app_state))]
pub async fn listen_for_proxies_connecting(
    pool: ProxyPool,
    proxy_listener: TcpListener,
    app_state: Arc<AppState>,
) -> () {
    while let Ok((stream, addr)) = proxy_listener.accept().await {
        let pool = pool.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            if let Err(err) = server::conn::http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |req| {
                        handle_proxy_request(pool.clone(), req, app_state.clone())
                    }),
                )
                .with_upgrades()
                .await
            {
                tracing::error!("Failed to serve connection from addr {:?}: {:?}", addr, err);
            }
        });
    }
}

#[tracing::instrument(name = "handle_proxy_request", skip(app_state), ret, err)]
async fn handle_proxy_request(
    pool: ProxyPool,
    mut req: Request<hyper::body::Incoming>,
    app_state: Arc<AppState>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    process_proxy_headers(app_state, &mut req).await?;
    if Method::CONNECT == req.method() {
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
                Err(e) => tracing::error!("upgrade error: {}", e),
            }
        });
        Ok(Response::new(empty()))
    } else {
        // TODO : Process request - can register proxy here
        tracing::info!("NOT CONNECT request");
        Ok(Response::new(empty()))
    }
}
